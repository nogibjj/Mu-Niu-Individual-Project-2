rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter


release:
	cargo build --release

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run:
	cargo run

test:
	cargo test --quiet