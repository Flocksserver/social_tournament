build:
	cargo build --release
	cargo test --release
	rm -R -f ./doc
	rustdoc src/lib.rs --crate-name social_tournament

publish:
	cargo publish