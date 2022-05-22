.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd lib && cargo +nightly fmt --all -- --check
	cd puredata && cargo +nightly fmt --all -- --check
	cd eurorack && cargo +nightly fmt --all -- --check
	cd peripherals && cargo +nightly fmt --all -- --check
	cd ui && cargo +nightly fmt --all -- --check

.PHONY: format
format:
	cd lib && cargo +nightly fmt --all
	cd puredata && cargo +nightly fmt --all
	cd eurorack && cargo +nightly fmt --all
	cd peripherals && cargo +nightly fmt --all
	cd ui && cargo +nightly fmt --all

.PHONY: clippy
clippy:
	cd lib && cargo +nightly clippy --all -- -D warnings
	cd puredata && cargo +nightly clippy --all -- -D warnings
	cd eurorack && cargo +nightly clippy --all -- -D warnings
	cd peripherals && cargo +nightly clippy --all --features defmt -- -D warnings
	cd ui && cargo +nightly clippy --all --features defmt -- -D warnings

.PHONY: test
test:
	cd lib && cargo +nightly test --all
	cd peripherals && cargo +nightly test --features defmt --all
	cd ui && cargo +nightly test --features defmt --all

.PHONY: update
update:
	cd lib && cargo +nightly update
	cd puredata && cargo +nightly update
	cd eurorack && cargo +nightly update
	cd peripherals && cargo +nightly update
	cd ui && cargo +nightly update

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
