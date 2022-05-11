.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd lib && cargo fmt --all -- --check
	cd puredata && cargo fmt --all -- --check
	cd eurorack && cargo fmt --all -- --check

.PHONY: format
format:
	cd lib && cargo fmt --all
	cd puredata && cargo fmt --all
	cd eurorack && cargo fmt --all

.PHONY: clippy
clippy:
	cd lib && cargo clippy --all -- -D warnings
	cd puredata && cargo clippy --all -- -D warnings
	cd eurorack && cargo clippy --all -- -D warnings

.PHONY: test
test:
	cd lib && cargo test --all

.PHONY: update
update:
	cd lib && cargo update
	cd puredata && cargo update
	cd eurorack && cargo update

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo build --release
	cp puredata/target/release/libproton_puredata.so ~/.local/lib/pd/extra/proton~.pd_linux
	pd puredata/proton.pd

.PHONY: test-embedded
test-embedded:
	cd eurorack && DEFMT_LOG=info cargo test --test integration

.PHONY: flash
flash:
	cd eurorack && cargo run --bin firmware $(FLAGS)

.PHONY: flash-dfu
flash-dfu:
	cd eurorack && cargo objcopy $(FLAGS) -- -O binary target/proton.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/proton.bin -d ,0483:df11

.PHONY: debug-test
debug-test:
	./hack/debug_test.sh
