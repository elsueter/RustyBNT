build&run: build run

build:
	cargo build

run:
	cargo run

release: build
	cargo run -r