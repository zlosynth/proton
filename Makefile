.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd lib && cargo fmt --all -- --check
	cd puredata && cargo fmt --all -- --check

.PHONY: format
format:
	cd lib && cargo fmt --all
	cd puredata && cargo fmt --all

.PHONY: clippy
clippy:
	cd lib && cargo clippy --all -- -D warnings
	cd puredata && cargo clippy --all -- -D warnings

.PHONY: test
test:
	cd lib && cargo test --all

.PHONY: update
update:
	cd lib && cargo update
	cd puredata && cargo update

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo build --release
	cp puredata/target/release/libproton_puredata.so ~/.local/lib/pd/extra/proton~.pd_linux
	pd puredata/proton.pd
