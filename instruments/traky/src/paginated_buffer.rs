//! Manipulate persistent buffers with lengths that are exceeding memory capacity.
//!
//! # TODO
//!
//! * Rename recording to writing
//! * Rename paginated buffer to something better... cursor? scroller?
//! * Rewrite this so it contains the whole subsystem of "buffer"
//! * Separate modules and unit tests for page, paginated_buffer, caller, cacher
//! * Root would contain e2e tests that would exercise the full subsystem
//! * Instead of running in multiple rutines, it would keep global queues and
//!   instances. It would then run every piece of the test in a scope {} and
//!   pass data over those queues
//! * SD card interaction would be also faked, but without much abstraction since
//!   proper SD abstraction with messages will be defined only later
//!
//! # Requirements
//!
//! * Can handle loops from 32 sample length up to tens of minutes.
//! * Allows simultaneous playback and recording.
//! * Recorded audio is being saved even while recording is in progress.
//! * Can immediatelly jump to the beginning of the sample and start playing.
//! * Saving and loading can be done in another routine.
//!
//! # Architecture
//!
//! * No pages are owned by the buffer.
//! * Caller is responsible for:
//!   * Providing new empty pages on request.
//!   * Persisting returned buffers.
//!   * Doing the two listed above with RT guarantees.
//! * Each of the page contains:
//!   * Fixed-size array of data.
//!   * "Dirty" flag.
//!   * Length of recorded data.
//!   * Start address, relative to the parent sample.
//!
//! # Flow starting from fresh
//!
//! 1. Caller initializes empty page on Pool and passes Box to the buffer.
//! 2. Buffer stores the Box in its struct.
//! 3. Caller passes input audio, info about armed channels.
//! 4. Buffer writes the audio into its active page and returns output audio.
//! 5. Caller asks the buffer whether it is full, if it is, it takes its page and passes
//!    a fresh one to it again.
//! 6. Since this was the first page, caller stores it in its cache.
//! 7. Caller passess the dirty page to SD save queue.
//! 8. Buffer continues recording, until its full again, swaps the page.
//! 9. Caller passes the dirty page to save queue.
//! 10. This continues for some time, until position reset is triggered.
//! 11. With reset armed, caller will force buffer to return its current buffer,
//!     and it will pass a clone of the start page to it.
//! 12. Caller recognizes that the next page is available on SD, it will send
//!     a request for SD loader to pull it. It should be eventually available
//!     in a loaded queue.
//! 13. The loaded page is then passed to buffer instead of empty pages used before.
//! 14. At some point, midway through the sample, recording stops.
//! 15. Any new samples will be returned like before, except now they will not
//!     be dirty and thus just thrown away.
//!
//! # Flow starting from a loaded sample
//!
//! 1. The caller recognizes there is a sample available and it reads its length.
//! 2. The caller loads the first page, queues fetching of the second one, if there is one.
//! 3. The caller passes the first page to the buffer.
//! 4. Business as usual.
//!
//! # Working with samples shorter than a single page
//!
//! 1. Buffer gets an inpulse to reset midway through the first page.
//! 2. The caller takes page from the buffer, clones it for save queue, clones it for
//!    its own cache and passes it back to the buffer.
//!
//! # Optimizations
//!
//! * There may be a queue of empty pages, initialized in the background so they
//!   are readily available.

mod sample;

const HARDCODED_PARENT: usize = 8;
const PAGE_SIZE: usize = 512;

struct Caller {
    buffer: Option<PaginatedBuffer>,
    page_1_cache: Option<Page>,
}

impl Caller {
    fn new() -> Self {
        Self {
            buffer: None,
            page_1_cache: None,
        }
    }

    fn set_sample(&mut self, sample: sample::Sample) {
        self.buffer = Some(PaginatedBuffer::from_sample(sample));
    }

    fn process_configuration_updates(
        &mut self,
        dsp_config_consumer: &mut heapless::spsc::Consumer<DSPConfig, 4>,
    ) {
        while let Some(dsp_config) = dsp_config_consumer.dequeue() {
            self.buffer
                .as_mut()
                .unwrap()
                .arm_recording(dsp_config.record());
        }
    }

    fn is_waiting_for_page(&self) -> bool {
        !self.buffer.as_ref().unwrap().has_page()
    }

    fn try_fetching_next_page(
        &mut self,
        load_response_consumer: &mut heapless::spsc::Consumer<Page, 4>,
    ) -> bool {
        let buffer = self.buffer.as_mut().unwrap();

        if buffer.wants_next().is_first() {
            buffer.set_page(self.page_1_cache.take().unwrap());
            return true;
        }

        while let Some(page) = load_response_consumer.dequeue() {
            if page.signature() == buffer.wants_next() {
                buffer.set_page(page);
                return true;
            }
        }

        false
    }

    fn start_loading_next_page(
        &mut self,
        load_request_producer: &mut heapless::spsc::Producer<PageRequest, 4>,
    ) {
        let buffer = self.buffer.as_mut().unwrap();
        let next_page_request = buffer.wants_next();
        load_request_producer
            .enqueue(next_page_request)
            .ok()
            .unwrap();
    }

    fn process(&mut self, recorded_audio: &mut [f32]) {
        self.buffer.as_mut().unwrap().process(recorded_audio);
    }

    fn has_full_page(&self) -> bool {
        self.buffer.as_ref().unwrap().is_page_full()
    }

    fn start_saving(&mut self, save_request_producer: &mut heapless::spsc::Producer<Page, 4>) {
        let buffer = self.buffer.as_mut().unwrap();

        let page = buffer.take_page();

        if page.index() == 0 {
            // TODO: Mark it in page memory as pinned, so it won't be
            // dropped after SD manager is done with it.
            self.page_1_cache = Some(page.clone());
        }

        if page.is_dirty() {
            save_request_producer.enqueue(page).ok().unwrap();
        }
    }

    fn reset_position(&mut self) {
        let buffer = self.buffer.as_mut().unwrap();
        buffer.reset_position();
    }
}

struct PaginatedBuffer {
    active_page: Option<Page>,
    recording: bool,
    pointer: usize,
    sample: sample::Sample,
}

impl PaginatedBuffer {
    fn from_sample(sample: sample::Sample) -> Self {
        Self {
            active_page: None,
            recording: false,
            pointer: 0, // Relative to sample
            sample,
        }
    }

    fn set_page(&mut self, page: Page) {
        self.active_page = Some(page);
    }

    fn take_page(&mut self) -> Page {
        self.active_page.take().unwrap()
    }

    fn has_page(&self) -> bool {
        self.active_page.is_some()
    }

    fn arm_recording(&mut self, record: bool) {
        self.recording = record;
    }

    fn process(&mut self, buffer: &mut [f32]) {
        // TODO: Assert multiples of 32 on buffers/pages
        if self.recording {
            self.active_page.as_mut().unwrap().is_dirty = true;
        }
        self.pointer += buffer.len();
        if self.pointer > self.sample.length {
            self.sample.length = self.pointer
        }
    }

    fn is_page_full(&self) -> bool {
        let page_pointer = self.pointer - (self.active_page.as_ref().unwrap().index() * PAGE_SIZE);
        page_pointer >= PAGE_SIZE
    }

    fn reset_position(&mut self) {
        self.pointer = 0;
    }

    fn wants_next(&self) -> PageRequest {
        if self.pointer == self.sample.length {
            PageRequest::Blank(HARDCODED_PARENT, self.pointer / PAGE_SIZE)
        } else {
            // TODO: Page len const
            PageRequest::Load(HARDCODED_PARENT, self.pointer / PAGE_SIZE)
        }
    }
}

#[derive(Clone)]
struct Page {
    parent_index: usize,
    page_index: usize,
    is_dirty: bool,
}

impl Page {
    fn new(parent_index: usize, page_index: usize) -> Self {
        Self {
            parent_index,
            page_index,
            is_dirty: false,
        }
    }
}

// Rename to PageSignature
#[derive(PartialEq, Eq)]
enum PageRequest {
    Load(usize, usize),
    Blank(usize, usize),
}

impl PageRequest {
    fn is_first(&self) -> bool {
        false
    }
}

struct DSPConfig {}

impl DSPConfig {
    fn new() -> Self {
        Self {}
    }

    fn record(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_flow_starting_from_nothing_with_long_recording() {
        use heapless::spsc::Queue;

        // TODO: Pass Page reference token (pointing to a place in a memory singleton)

        let mut save_request_queue: Queue<Page, 4> = Queue::new();
        let (mut save_request_producer, mut save_request_consumer) = save_request_queue.split();

        let mut load_request_queue: Queue<PageRequest, 4> = Queue::new();
        let (mut load_request_producer, mut load_request_consumer) = load_request_queue.split();

        let mut load_response_queue: Queue<Page, 4> = Queue::new();
        let (mut load_response_producer, mut load_response_consumer) = load_response_queue.split();

        let mut dsp_config_queue: Queue<DSPConfig, 4> = Queue::new();
        let (mut dsp_config_producer, mut dsp_config_consumer) = dsp_config_queue.split();

        // Owned by SD manager.
        let mut sd: [Option<Page>; 4] = [None, None, None, None];

        // Owned by the caller. Running as DSP loop.
        let mut caller = Caller::new();

        // Loading metadata about the selected sample from SD.
        // This will be solely based on the length of the file found on the file
        // system. There should be no metadata saved on side.
        caller.set_sample(sample::Sample::new());
        caller.start_loading_next_page(&mut load_request_producer);

        // SD Manager initializing the page and passing it to the caller.
        {
            let request = load_request_consumer
                .dequeue()
                .expect("Load request must be received");
            assert!(
                matches!(request, PageRequest::Blank(HARDCODED_PARENT, 0)),
                "The first request must be for a blank page"
            );
            load_response_producer
                .enqueue(Page::new(HARDCODED_PARENT, 0))
                .ok()
                .unwrap();
        }

        // Control loop issues request for recording.
        {
            dsp_config_producer.enqueue(DSPConfig::new()).ok().unwrap();
        }

        // Caller records into the first page until its full. This would span multiple
        // DSP ticks.
        loop {
            caller.process_configuration_updates(&mut dsp_config_consumer);

            if caller.is_waiting_for_page() {
                let acquired = caller.try_fetching_next_page(&mut load_response_consumer);
                if acquired {
                    caller.start_loading_next_page(&mut load_request_producer);
                }
            }

            caller.process(&mut [0.0; 32]);

            if caller.has_full_page() {
                caller.start_saving(&mut save_request_producer);
                break;
            }
        }

        // SD manager
        {
            let _request = load_request_consumer.dequeue().unwrap();
            load_response_producer
                .enqueue(Page::new(HARDCODED_PARENT, 1))
                .ok()
                .unwrap();

            let page_1 = save_request_consumer.dequeue().unwrap();
            sd[0] = Some(page_1);
        }

        // Caller records into the second page until its full. This would span multiple
        // DSP ticks.
        loop {
            caller.process_configuration_updates(&mut dsp_config_consumer);

            if caller.is_waiting_for_page() {
                let acquired = caller.try_fetching_next_page(&mut load_response_consumer);
                if acquired {
                    caller.start_loading_next_page(&mut load_request_producer);
                }
            }

            caller.process(&mut [0.0; 32]);

            if caller.has_full_page() {
                caller.start_saving(&mut save_request_producer);
                break;
            }
        }

        // SD manager
        {
            let _request = load_request_consumer.dequeue().unwrap();
            load_response_producer
                .enqueue(Page::new(HARDCODED_PARENT, 2))
                .ok()
                .unwrap();

            let page_2 = save_request_consumer.dequeue().unwrap();
            sd[1] = Some(page_2);
        }

        // Caller records into the third page, but is interrupted with a position reset.
        {
            for _ in 0..3 {
                caller.process_configuration_updates(&mut dsp_config_consumer);

                if caller.is_waiting_for_page() {
                    let acquired = caller.try_fetching_next_page(&mut load_response_consumer);
                    if acquired {
                        caller.start_loading_next_page(&mut load_request_producer);
                    }
                }

                caller.process(&mut [0.0; 32]);
            }

            caller.start_saving(&mut save_request_producer);
            caller.reset_position();
        }

        // SD manager
        {
            let _request = load_request_consumer.dequeue().unwrap();
            load_response_producer
                .enqueue(Page::new(HARDCODED_PARENT, 1))
                .ok()
                .unwrap();

            let page_3 = save_request_consumer.dequeue().unwrap();
            sd[2] = Some(page_3);
        }

        // Caller records into the first page again
        loop {
            caller.process_configuration_updates(&mut dsp_config_consumer);

            if caller.is_waiting_for_page() {
                let acquired = caller.try_fetching_next_page(&mut load_response_consumer);
                if acquired {
                    caller.start_loading_next_page(&mut load_request_producer);
                }
            }

            caller.process(&mut [0.0; 32]);

            if caller.has_full_page() {
                caller.start_saving(&mut save_request_producer);
                break;
            }
        }

        // SD manager stores the first page and returns next.
        {
            let page_1 = save_request_consumer.dequeue().unwrap();
            sd[0] = Some(page_1);

            let _request = load_request_consumer.dequeue().unwrap();
            load_response_producer
                .enqueue(sd[1].clone().unwrap())
                .ok()
                .unwrap();
        }

        // Control loop issues request for recording.
        {
            dsp_config_producer.enqueue(DSPConfig::new()).ok().unwrap();
        }

        // Caller records into the first page until its full. This would span multiple
        // DSP ticks.
        loop {
            caller.process_configuration_updates(&mut dsp_config_consumer);

            if caller.is_waiting_for_page() {
                let acquired = caller.try_fetching_next_page(&mut load_response_consumer);
                if acquired {
                    caller.start_loading_next_page(&mut load_request_producer);
                }
            }

            caller.process(&mut [0.0; 32]);

            if caller.has_full_page() {
                caller.start_saving(&mut save_request_producer);
                break;
            }
        }
    }

    #[test]
    fn full_flow_starting_with_existing_long_sample() {}

    #[test]
    fn full_flow_starting_from_nothing_with_short_sample() {}
}
