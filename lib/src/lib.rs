#![no_std]

extern crate alloc;

#[macro_use]
extern crate graphity;

#[cfg(test)]
#[macro_use]
extern crate approx;

// TODO: Make it private out it has a user in the package
pub mod engine;

mod modules;
mod primitives;
