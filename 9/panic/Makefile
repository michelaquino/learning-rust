## Compile the code and generate a binary to ./target/debug/hello_cargo
build-local:
	cargo build

## Compile the code with all optimizations and generate a binary to ./target/release/hello_cargo
release:
	cargo build --release

## Check the code, but don't generate a binary
check:
	cargo check

## Compile and run
run: 
	cargo run

## Compile and run with backtrace
run-with-backtrace: 
	# RUST_BACKTRACE=full cargo run
	RUST_BACKTRACE=1 cargo run