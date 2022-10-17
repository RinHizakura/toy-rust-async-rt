all: build run

build:
	cargo build

run:
	cargo r --example main

clean:
	cargo clean
