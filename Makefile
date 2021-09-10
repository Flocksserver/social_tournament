build:
	cargo build --release
	cargo test --release
	cargo doc

publish:
	make build
	cargo publish