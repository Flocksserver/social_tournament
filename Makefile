build:
	cargo build --release
	cargo test --release

publish:
	make build
	cargo publish