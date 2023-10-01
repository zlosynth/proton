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

struct PaginatedBuffer {
    active_page: Option<Page>,
    recording: bool,
}

impl PaginatedBuffer {
    fn new() -> Self {
        Self {
            active_page: None,
            recording: false,
        }
    }

    fn set_page(&mut self, page: Page) {
        self.active_page = Some(page);
    }

    fn take_page(&mut self) -> Page {
        self.active_page.take().unwrap()
    }

    fn arm_recording(&mut self, record: bool) {
        self.recording = record;
    }

    fn process(&self, _buffer: &mut [f32]) {}

    fn is_page_full(&self) -> bool {
        false
    }

    fn reset_position(&self) {}
}

#[derive(Clone)]
struct Page {}

impl Page {
    fn new() -> Self {
        Self {}
    }

    fn is_dirty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_flow_starting_from_nothing_with_long_recording() {
        // TODO: Pass Page reference token (pointing to a place in a memory singleton)
        let mut sd: [Option<Page>; 4] = [None, None, None, None];
        let mut page_1_cache: Option<Page> = None;

        let mut buffer = PaginatedBuffer::new();

        // 1. Caller initializes empty page on Pool and passes Box to the buffer.
        // 2. Buffer stores the Box in its struct.
        let page_1 = Page::new();
        buffer.set_page(page_1);

        // 3. Caller passes input audio, info about armed channels.
        // 4. Buffer writes the audio into its active page and returns output audio.
        // 5. Caller asks the buffer whether it is full, if it is, it takes its page and passes
        //    a fresh one to it again.
        buffer.arm_recording(true);
        let mut recorded_audio = [0.0; 32];
        loop {
            buffer.process(&mut recorded_audio);
            if buffer.is_page_full() {
                break;
            }
        }
        let page_1 = buffer.take_page();
        let page_2 = Page::new();
        buffer.set_page(page_2);

        // 6. Since this was the first page, caller stores it in its cache.
        // 7. Caller passess the dirty page to SD save queue.
        if page_1.is_dirty() {
            sd[0] = Some(page_1.clone());
            page_1_cache = Some(page_1);
        }

        // 8. Buffer continues recording, until its full again, swaps the page.
        // 9. Caller passes the dirty page to save queue.
        buffer.arm_recording(true);
        let mut recorded_audio = [0.0; 32];
        loop {
            buffer.process(&mut recorded_audio);
            if buffer.is_page_full() {
                break;
            }
        }
        let page_2 = buffer.take_page();
        let page_3 = Page::new();
        buffer.set_page(page_3);
        if page_2.is_dirty() {
            sd[1] = Some(page_2);
        }

        // 10. This continues for some time, until position reset is triggered.
        buffer.arm_recording(true);
        let mut recorded_audio = [0.0; 32];
        buffer.process(&mut recorded_audio);
        buffer.process(&mut recorded_audio);
        buffer.process(&mut recorded_audio);
        buffer.reset_position();

        // 11. With reset armed, caller will force buffer to return its current buffer,
        //     and it will pass a clone of the start page to it.
        let page_3 = buffer.take_page();
        if page_3.is_dirty() {
            sd[2] = Some(page_3);
        }
        buffer.set_page(page_1_cache.unwrap());

        // 12. Caller recognizes that the next page is available on SD, it will send
        //     a request for SD loader to pull it. It should be eventually available
        //     in a loaded queue.
        let page_2 = sd[1].clone().unwrap();

        // 13. The loaded page is then passed to buffer instead of empty pages used before.
        buffer.set_page(page_2);

        // 14. At some point, midway through the sample, recording stops.
        buffer.arm_recording(true);
        let mut recorded_audio = [0.0; 32];
        buffer.process(&mut recorded_audio);
        buffer.process(&mut recorded_audio);
        buffer.arm_recording(false);

        // 15. Any new samples will be returned like before, except now they will not
        //     be dirty and thus just thrown away.
    }

    #[test]
    fn full_flow_starting_with_existing_long_sample() {}

    #[test]
    fn full_flow_starting_from_nothing_with_short_sample() {}
}
