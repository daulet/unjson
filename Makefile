.PHONY: release

release:
	cargo build --release --target=x86_64-apple-darwin
	cargo build --release --target=aarch64-apple-darwin
	mkdir -p release
	cp target/x86_64-apple-darwin/release/unjson release/unjson-darwin-x86_64
	cp target/aarch64-apple-darwin/release/unjson release/unjson-darwin-aarch64
	openssl dgst -sha256 release/unjson-darwin-x86_64
	openssl dgst -sha256 release/unjson-darwin-aarch64
