build:
	cargo build

test: build
	./test.sh

clean:
	cargo clean

.PHONY: build test clean
