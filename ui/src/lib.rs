#![no_std]

pub mod action;
pub mod display;
pub mod input;
pub mod view;

// TODO NOTE
// user input passes actions over a queue to state reducer
// state reducer keeps State with Vecs etc
// state reducer passes display state (Copy) via a #[task] argument to display
// state reducer passes actions via queue to lib front, that handles interporation
// CV input passes actions via queue to lib front
// audio loop is owned by the lib itself
