//! Manipulate persistent buffers with lengths that are exceeding memory capacity.
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

// const MINIMAL_BLOCK_LENGTH = 32;

// type Frame = ((f32, f32), (f32, f32), (f32, f32), (f32, f32));

struct DynamicBuffer {
    // first_block: Block,
    // // TODO: Keep moving blocks outside, so the dynamic buffer can be used
    // // with reference to populated block while the other one is being populated.
    // moving_blocks: (Block, Block),
    // active_block: ActiveBlock,
    // active_block_position: usize,
    // sample_length: usize,
}

impl DynamicBuffer {
    fn new() -> Self {
        Self {}
    }

    fn load(&mut self) {}
}

// struct Block {
//     length: (),
//     buffer: (),
// }

// enum ActiveBlock {
//     First,
//     Low,
//     High,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_end_to_end() {
        let mut dynamic_buffer = DynamicBuffer::new();

        let save_exists = true;
        if save_exists {
            dynamic_buffer.load();
        }

        // TODO: play two passes
        // TODO: start recording while playing, pass input and say to which channels it should go
        // TODO: stop recording
        // TODO: play two passes, this time with recorded stuff too
    }
}
