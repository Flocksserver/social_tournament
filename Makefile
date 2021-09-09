build:
	cargo build --release
	cargo test --release
	rm -R -f ./doc
	rustdoc src/lib.rs --crate-name social_tournament

publish:
	make build
	cargo publish