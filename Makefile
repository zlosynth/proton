.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd puredata && cargo +nightly fmt --all -- --check
	cd eurorack && cargo +nightly fmt --all -- --check
	cd peripherals && cargo +nightly fmt --all -- --check
	cd ui && cargo +nightly fmt --all -- --check
	cd primitives && cargo +nightly fmt --all -- --check

.PHONY: format
format:
	cd puredata && cargo +nightly fmt --all
	cd eurorack && cargo +nightly fmt --all
	cd peripherals && cargo +nightly fmt --all
	cd ui && cargo +nightly fmt --all
	cd primitives && cargo +nightly fmt --all

.PHONY: clippy
clippy:
	cd puredata && cargo +nightly clippy --all -- -D warnings
	cd eurorack && cargo +nightly clippy --all -- -D warnings
	cd eurorack && cargo +nightly check --test display --test encoders
	cd peripherals && cargo +nightly clippy --all --features defmt -- -D warnings
	cd ui && cargo +nightly clippy --all --features defmt -- -D warnings
	cd primitives && cargo +nightly clippy --all -- -D warnings

.PHONY: test
test:
	cd peripherals && cargo +nightly test --features defmt --all
	cd ui && cargo +nightly test --features defmt --all
	cd primitives && cargo +nightly test --all

.PHONY: update
update:
	cd puredata && cargo +nightly update
	cd eurorack && cargo +nightly update
	cd peripherals && cargo +nightly update
	cd ui && cargo +nightly update
	cd primitives && cargo +nightly update

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo +nightly build --release
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
	cd eurorack && cargo +nightly run --bin firmware $(FLAGS)

.PHONY: flash-dfu
flash-dfu:
	cd eurorack && cargo +nightly objcopy $(FLAGS) -- -O binary target/proton.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/proton.bin -d ,0483:df11

.PHONY: debug-test
debug-test:
	WHAT=$(WHAT) ./hack/debug_test.sh
