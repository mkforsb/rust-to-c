.PHONY: leakcheck

leakcheck:
	cargo build
	valgrind ./target/debug/rust-app
