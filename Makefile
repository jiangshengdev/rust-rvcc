build:
	@cargo build -q

test: build
	@./test.sh

clean:
	@cargo clean

.PHONY: build test clean
