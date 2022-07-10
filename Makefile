INSTRUMENT ?= tape

.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd puredata && cargo +nightly fmt --all -- --check
	cd eurorack && cargo +nightly fmt --all -- --check
	cd peripherals && cargo +nightly fmt --all -- --check
	cd ui && cargo +nightly fmt --all -- --check
	cd control && cargo +nightly fmt --all -- --check
	cd primitives && cargo +nightly fmt --all -- --check
	cd instruments/karplus_strong && cargo +nightly fmt --all -- --check
	cd instruments/tape && cargo +nightly fmt --all -- --check

.PHONY: format
format:
	cd puredata && cargo +nightly fmt --all
	cd eurorack && cargo +nightly fmt --all
	cd peripherals && cargo +nightly fmt --all
	cd ui && cargo +nightly fmt --all
	cd control && cargo +nightly fmt --all
	cd primitives && cargo +nightly fmt --all
	cd instruments/karplus_strong && cargo +nightly fmt --all
	cd instruments/tape && cargo +nightly fmt --all

.PHONY: clippy
clippy:
	cd puredata && cargo +nightly clippy --all --features tape -- -D warnings
	cd puredata && cargo +nightly clippy --all --features karplus_strong -- -D warnings
	cd eurorack && cargo +nightly clippy --all --features tape -- -D warnings
	cd eurorack && cargo +nightly clippy --all --features karplus_strong -- -D warnings
	cd eurorack && cargo +nightly check --test display --test encoders --features karplus_strong
	cd peripherals && cargo +nightly clippy --all --features defmt -- -D warnings
	cd ui && cargo +nightly clippy --all --features defmt -- -D warnings
	cd control && cargo +nightly clippy --all --features defmt -- -D warnings
	cd primitives && cargo +nightly clippy --all --features defmt -- -D warnings
	cd instruments/karplus_strong && cargo +nightly clippy --all --features defmt -- -D warnings
	cd instruments/tape && cargo +nightly clippy --all --features defmt -- -D warnings
	cd instruments/tape && cargo +nightly check --benches --all

.PHONY: test
test:
	cd peripherals && cargo +nightly test --features defmt --all
	cd ui && cargo +nightly test --features defmt --all
	cd control && cargo +nightly test --features defmt --all
	cd primitives && cargo +nightly test --all --features defmt
	cd instruments/karplus_strong && cargo +nightly test --all --features defmt
	cd instruments/tape && cargo +nightly test --all --features defmt

.PHONY: update
update:
	cd puredata && cargo +nightly update
	cd eurorack && cargo +nightly update
	cd peripherals && cargo +nightly update
	cd ui && cargo +nightly update
	cd control && cargo +nightly update
	cd primitives && cargo +nightly update
	cd instruments/karplus_strong && cargo +nightly update
	cd instruments/tape && cargo +nightly update

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo +nightly build --release --features $(INSTRUMENT)
	cp puredata/target/release/libproton_puredata.so ~/.local/lib/pd/extra/proton~.pd_linux
	pd puredata/proton.pd

.PHONY: test-embedded
test-embedded:
	cd eurorack && DEFMT_LOG=info cargo +nightly test --test display
	cd eurorack && DEFMT_LOG=info cargo +nightly test --test encoders

.PHONY: test-ui
test-ui:
	cd ui && cargo +nightly run --example display

.PHONY: flash
flash:
	cd eurorack && cargo +nightly run --bin firmware $(FLAGS) --features $(INSTRUMENT)

.PHONY: flash-dfu
flash-dfu:
	cd eurorack && cargo +nightly objcopy $(FLAGS) --features $(INSTRUMENT) -- -O binary target/proton.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/proton.bin -d ,0483:df11

.PHONY: debug-test
debug-test:
	WHAT=$(WHAT) ./hack/debug_test.sh
