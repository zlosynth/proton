#![no_std]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod ad_envelope;
pub mod state_variable_filter;
pub mod white_noise;

#[cfg(test)]
mod spectral_analysis;
