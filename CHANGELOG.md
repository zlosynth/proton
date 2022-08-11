# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

## 0.6.0

* Transform the module into two-board design.
* Remove MIDI input and output.
* Implement a basic tape simulation instrument.
* Introduce oversampling converters over the Signal trait.
* Reconcile CV inputs and pass them to instruments.
* Allow control of cutoff and feedback of karplus strong via CV.

## 0.5.0

* Implement sound output in the firmware.
* Introduce a basic instrument based around Karplus Strong algorithm.
* Allow custom settings for f32 attribute range.
* Implement a simple turing machine-like sequencer.

## 0.4.0

* Integrate display in the firmware.
* Use left encoder to move through attributes.
* Use right encoder to adjust attributes.

## 0.3.0

* Introduce support for module instantiation and removal.
* Implement menu for dynamic connecting and disconnecting of patches.

## 0.2.0

* Introduce signal buffer abstraction.
* Abstract cells used to access internal audio output and control input data.
* Implement read-only UI for 128x64 OLED displays.

## 0.1.0

* Initialize the project with READMEs and licensing.
* Add schetch of the front panel.
* Prepare schematics of the module.
* Design the PCB of the module.
* Implement basic saw oscillator.
* Introduce pure data external for testing.
