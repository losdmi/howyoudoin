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
test:
	cargo test
