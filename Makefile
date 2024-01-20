all: build

build:
	cargo build

run:
	dx serve --hot-reload --platform desktop

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean
