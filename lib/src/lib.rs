#![no_std]

extern crate alloc;

#[macro_use]
extern crate graphity;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod instrument;

mod core;
mod modules;
mod primitives;
