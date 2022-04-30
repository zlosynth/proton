const BUFFER_SIZE: usize = 32;
type Unit = f32;
type Buffer = [Unit; BUFFER_SIZE];

#[derive(Default, Clone, Copy)]
pub struct Signal {
    buffer: Buffer,
}

impl Signal {
    pub fn from_audio(buffer: Buffer) -> Self {
        Self { buffer }
    }

    pub fn from_control(unit: Unit) -> Self {
        Self {
            buffer: [unit; BUFFER_SIZE],
        }
    }

    pub fn as_audio(self) -> Buffer {
        self.buffer
    }

    pub fn as_control(self) -> Unit {
        self.buffer[0]
    }
}
