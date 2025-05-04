.PHONY: run
run: lint test
	cargo run

.PHONY: run_raw
run_raw:
	cargo run

.PHONY: lint
lint:
	cargo clippy

.PHONY: test
test: lint
	cargo test

.PHONY: build_for_windows
build_for_windows:
	cargo build --target x86_64-pc-windows-gnu --release
