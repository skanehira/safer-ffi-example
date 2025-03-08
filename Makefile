build:
	@cargo build --release
	@cargo run --features headers --bin generate-headers

run: build
	@cd go_example && go run .

lib-test:
	@cargo test

test: lib-test
	@cd go_example && go test -v ./...
