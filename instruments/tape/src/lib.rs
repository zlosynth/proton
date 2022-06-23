#![no_std]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod instrument;

pub use instrument::Instrument;
