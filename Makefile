build:
	cargo build
run:
	cargo run
clippy:
	cargo clippy -- -D warnings
fmt:
	cargo fmt --check
test:
	cargo test
