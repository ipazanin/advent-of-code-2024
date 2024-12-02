######################################################
#                       Basic
######################################################
start::
	cargo run

build::
	cargo build

check::
	cargo check
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

test::
	 cargo test

format::
	cargo clippy --fix
	cargo fmt --all

clean::
	rm -rf ./target

update::
	rustup update
	cargo update