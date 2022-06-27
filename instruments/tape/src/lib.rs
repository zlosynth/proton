#![no_std]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod instrument;

mod hysteresis;

pub use instrument::Instrument;

pub trait Rand {
    fn generate(&mut self) -> u16;
}
