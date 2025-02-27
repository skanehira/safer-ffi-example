gen-header: build
	@cargo run --features headers --bin generate-headers

build:
	@cargo build --release
