TARGET ?= target/debug/elf-parser-in-rust-book

run: format test target/debug/elf-parser-in-rust-book
	cargo run $(TARGET)

target/debug/elf-parser-in-rust-book:
	cargo build


format:
	cargo fmt

test:
	cargo test
