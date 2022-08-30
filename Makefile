INSTRUMENT ?= kaseta

.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd puredata && cargo fmt --all -- --check
	cd eurorack && cargo fmt --all -- --check
	cd peripherals && cargo fmt --all -- --check
	cd ui && cargo fmt --all -- --check
	cd control && cargo fmt --all -- --check
	cd instruments/kaseta && cargo fmt --all -- --check

.PHONY: format
format:
	cd puredata && cargo fmt --all
	cd eurorack && cargo fmt --all
	cd peripherals && cargo fmt --all
	cd ui && cargo fmt --all
	cd control && cargo fmt --all
	cd instruments/kaseta && cargo fmt --all

.PHONY: clippy
clippy:
	cd puredata && cargo clippy --all --features kaseta -- -D warnings
	cd eurorack && cargo clippy --all --features kaseta -- -D warnings
	cd peripherals && cargo clippy --all --features defmt -- -D warnings
	cd ui && cargo clippy --all --features defmt -- -D warnings
	cd control && cargo clippy --all --features defmt -- -D warnings
	cd instruments/kaseta && cargo clippy --all --features defmt -- -D warnings
	cd instruments/kaseta && cargo check --benches --all

.PHONY: test
test:
	cd peripherals && cargo test --features defmt --all
	cd ui && cargo test --features defmt --all
	cd control && cargo test --features defmt --all
	cd instruments/kaseta && cargo test --all --features defmt

.PHONY: update
update:
	cd puredata && cargo update
	cd eurorack && cargo update
	cd peripherals && cargo update
	cd ui && cargo update
	cd control && cargo update
	cd instruments/kaseta && cargo update

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo build --release --features $(INSTRUMENT)
	cp puredata/target/release/libproton_puredata.so ~/.local/lib/pd/extra/proton~.pd_linux
	pd puredata/proton.pd

.PHONY: test-embedded
test-embedded:
	cd eurorack && DEFMT_LOG=info cargo test --test encoder --features $(INSTRUMENT)
	cd eurorack && DEFMT_LOG=info cargo test --test display --features $(INSTRUMENT)
	cd eurorack && DEFMT_LOG=info cargo test --test cv_input --features $(INSTRUMENT)
	cd eurorack && DEFMT_LOG=info cargo test --test gate_output --features $(INSTRUMENT)

.PHONY: test-ui
test-ui:
	cd ui && cargo run --example display

.PHONY: flash
flash:
	cd eurorack && cargo run --bin firmware $(FLAGS) --features $(INSTRUMENT)

.PHONY: flash-dfu
flash-dfu:
	cd eurorack && cargo objcopy $(FLAGS) --features $(INSTRUMENT) -- -O binary target/proton.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/proton.bin -d ,0483:df11

.PHONY: debug-test
debug-test:
	WHAT=$(WHAT) ./hack/debug_test.sh
