# Development

## Environment setup

Follow [Rust's installation guide](https://www.rust-lang.org/tools/install).

Install tooling of the embedded Rust target for Cortex-M7F:

```sh
rustup target add thumbv7em-none-eabihf
```

## Formatting, linting, unit tests

Run formatting, linter and unit tests:

```sh
make
```

Read the Makefile to learn more.

## Benchmark

If a package has a benchmark defined, `cd` into its directory and run it via:

``` sh
cargo +nightly bench --bench bench
```

Use the benchmark for profiling:

``` sh
rm -f target/release/deps/bench-*
rm -f callgrind.out.*
RUSTFLAGS="-g" cargo +nightly bench --bench bench --no-run
BENCH=$(find target/release/deps -type f -executable -name 'bench-*')
TEST=instrument
valgrind \
    --tool=callgrind \
    --dump-instr=yes \
    --collect-jumps=yes \
    --simulate-cache=yes \
    ${BENCH} --bench --profile-time 10 ${TEST}
kcachegrind callgrind.out.*
```


## Flash via ST-Link

This requires external probe, such as the ST LINK-V3 MINI. The benefit of this
approach is that it allows to stay connected to the module, read logs, run a
debugger, or execute tests on the module. Note that the module needs to be
powered while the probe is connected.

This project uses [probe-rs](https://github.com/probe-rs/probe-rs) to deal with
flashing. Start by installing its dependencies. For Fedora, it can be done by
running the following:

```sh
sudo dnf install -y libusbx-devel libftdi-devel libudev-devel
```

You may then install needed udev rules. See the [probe-rs getting
started](https://probe.rs/docs/getting-started/probe-setup/) to learn how.

Then install Rust dependencies of probe-rs:

```sh
cargo install probe-run
cargo install flip-link
```

To flash the project, call this make target:

```sh
make flash
```

Logging level can be set using an environment variable:

```sh
DEFMT_LOG=info make flash
```

To flash a release build, set a flag:

```sh
make flash FLAGS="--release"
```

Some tests must be executed directly on the module. To run those, use the
following target:

```sh
make test-embedded
```

## Flash via DFU

Unlike ST-Link, DFU flashing does not require any external probe. Just connect
the module to your computer via a USB cable.

First, install [dfu-util](http://dfu-util.sourceforge.net/). On Fedora, this can
be done by calling:

```sh
sudo dnf install dfu-util
```

To flash the project, call this make target:

```sh
make flash-dfu
```

To flash a release build, extend the command with a flag:

```sh
make flash-dfu FLAGS="--release"
```

## Firmware size

Check firmware size:

```sh
cd eurorack && cargo +nightly size --bin firmware --release -- -m # or -A
```

## Run a debugger

I prefer to use [gdbgui](https://www.gdbgui.com/) when I need to attach a
debugger to a test run.

First, install gdbgui following its [installation
guide](https://www.gdbgui.com/installation/). The make target also requires `jq`
to be installed in the system.

To attach the debugger to a unit test run:

```sh
make debug-test WHAT=ui
```

After the debugger is open, set breakpoints, type `r` into the GDB console and
click Continue.

## Pure data

To try out the project via Pure Data:

```sh
make puredata
```

And more under `hack/` and in the `Makefile`.

## Gerbers, BOM and CPL

I extensivelly use https://github.com/Bouni/kicad-jlcpcb-tools to deal with the
matters listed in the title, and to prepare project for manufacture.
