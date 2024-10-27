rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter


install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

release:
	cd psr && cargo build --release

format:
	cd psr && cargo fmt --quiet

lint:
	cd psr && cargo clippy --quiet

run:
	cd psr && cargo run

test:
	cd psr && cargo test --quiet