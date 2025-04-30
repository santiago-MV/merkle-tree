build:
	cargo build
clippy:
	cargo clippy -- -D warnings
fmt:
	cargo fmt --check
test:
	cargo test
