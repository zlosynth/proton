#![no_std]

mod paginated_buffer;

use core::convert::TryFrom;
use core::fmt;

use micromath::F32Ext as _;

use embedded_sdmmc::blockdevice::BlockDevice;
use embedded_sdmmc::{Controller, Mode, VolumeIdx};
use proton_control::input_snapshot::InputSnapshot;
use proton_instruments_interface::{
    Instrument as InstrumentTrait, MemoryManager, Rand as ProtonRandomizer,
};
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Traky";
const VOLUME_ATTRIBUTE: &str = "volume";

const MAX_SAMPLE_LENGTH_IN_SECONDS: u32 = 20;

pub struct Instrument {
    sample: Sample,
    pointer: f32,
    speed: f32,
}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl InstrumentTrait for Instrument {
    type Command = Command;

    fn new(
        sample_rate: u32,
        memory_manager: &mut MemoryManager,
        sd: &mut impl BlockDevice<Error = impl core::fmt::Debug>,
    ) -> Self {
        defmt::info!("Allocating buffer");
        let mut sample = prepare_empty_sample(memory_manager, sample_rate);

        defmt::info!("Loading sample from SD");
        load_sample_from_sd(sd, &mut sample);

        defmt::info!("Initialization complete");

        Self {
            sample,
            pointer: 0.0,
            speed: 0.0,
        }
    }

    fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[Attribute::new(VOLUME_ATTRIBUTE)
                .with_value_f32(ValueF32::new(1.0).with_writter(writter))])
            .unwrap()
    }

    fn process(&mut self, buffer: &mut [(f32, f32)], _randomizer: &mut impl ProtonRandomizer) {
        for tuple in buffer.iter_mut() {
            let pointer_usize = self.pointer as usize;
            let tuple_a = self.sample.buffer[pointer_usize];
            let tuple_b = if pointer_usize + 1 >= self.sample.length {
                self.sample.buffer[0]
            } else {
                self.sample.buffer[pointer_usize + 1]
            };

            let new_tuple = {
                (
                    tuple_a.0 + (tuple_b.0 - tuple_a.0) * self.pointer.fract(),
                    tuple_a.1 + (tuple_b.1 - tuple_a.1) * self.pointer.fract(),
                )
            };
            *tuple = new_tuple;

            self.pointer += 0.5 + 0.5 * self.speed;
            if self.pointer as usize >= self.sample.length {
                self.pointer = self.pointer.fract();
            }
        }
    }

    fn execute(&mut self, _command: Command) {}

    fn update_control(&mut self, snapshot: InputSnapshot) {
        self.speed = 1.0 - snapshot.pot.value;
    }
}

fn prepare_empty_sample(memory_manager: &mut MemoryManager, sample_rate: u32) -> Sample {
    Sample::from_buffer(
        memory_manager
            .allocate(upper_power_of_two(2 * sample_rate * MAX_SAMPLE_LENGTH_IN_SECONDS) as usize)
            .unwrap(),
    )
}

fn load_sample_from_sd(
    sd: &mut impl BlockDevice<Error = impl core::fmt::Debug>,
    sample: &mut Sample,
) {
    let mut fat = Controller::new(sd, TimeSource);
    let mut volume = fat.get_volume(VolumeIdx(0)).unwrap();
    let root_dir = fat.open_root_dir(&volume).unwrap();
    let mut file = fat
        .open_file_in_dir(&mut volume, &root_dir, "project.raw", Mode::ReadOnly)
        .unwrap();

    let mut buffer = [0u8; 512 * 2 * 64];
    while !file.eof() {
        let num_read = fat.read(&volume, &mut file, &mut buffer).unwrap();
        let buffer_tuple = {
            let pointer = &buffer as *const _ as *const (f32, f32);
            let buffer_tuple =
                unsafe { core::slice::from_raw_parts::<(f32, f32)>(pointer, num_read / 8) };
            buffer_tuple
        };
        for pair in buffer_tuple {
            sample.buffer[sample.length] = *pair;
            sample.length += 1;
        }
    }

    fat.close_file(&volume, file).unwrap();
    fat.close_dir(&volume, root_dir);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetVolume(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = ();

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(VOLUME_ATTRIBUTE, value) => Ok(Command::SetVolume(value)),
            _ => Err(()),
        }
    }
}

struct Sample {
    pub buffer: &'static mut [(f32, f32)],
    pub length: usize,
}

impl Sample {
    fn from_buffer(buffer_f32: &'static mut [f32]) -> Self {
        let pointer = buffer_f32 as *const _ as *mut (f32, f32);
        let buffer_tuple =
            unsafe { core::slice::from_raw_parts_mut::<(f32, f32)>(pointer, buffer_f32.len() / 2) };
        Self {
            buffer: buffer_tuple,
            length: 0,
        }
    }
}

fn upper_power_of_two(mut n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n += 1;
    n
}

pub struct TimeSource;

// This is just a placeholder TimeSource. In a real world application
// one would probably use the RTC to provide time.
impl embedded_sdmmc::TimeSource for TimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
