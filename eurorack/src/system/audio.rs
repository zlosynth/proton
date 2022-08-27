use daisy::audio::{self, Interface};

pub const SAMPLE_RATE: u32 = audio::FS.to_Hz();
pub const BLOCK_LENGTH: usize = audio::BLOCK_LENGTH;

pub struct Audio {
    interface: Option<Interface>,
}

impl Audio {
    pub fn init(interface: daisy::audio::Interface) -> Self {
        Self {
            interface: Some(interface),
        }
    }

    pub fn spawn(&mut self) {
        self.interface = Some(self.interface.take().unwrap().spawn().unwrap());
    }

    pub fn update_buffer(&mut self, callback: impl FnMut(&mut [(f32, f32); BLOCK_LENGTH])) {
        self.interface
            .as_mut()
            .unwrap()
            .handle_interrupt_dma1_str1(callback)
            .unwrap();
    }
}
