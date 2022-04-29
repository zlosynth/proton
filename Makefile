.PHONY: all
all: format clippy test

.PHONY: check-format
check-format:
	cd lib && cargo fmt --all -- --check

.PHONY: format
format:
	cd lib && cargo fmt --all

.PHONY: clippy
clippy:
	cd lib && cargo clippy --all -- -D warnings

.PHONY: test
test:
	cd lib && cargo test --all

.PHONY: update
update:
	cd lib && cargo update
