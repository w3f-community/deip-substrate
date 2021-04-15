.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: run
run:
	 cargo run --release -- --dev --tmp -lruntime=debug

.PHONY: build
build:
	 cargo build --release


.PHONY: doc
doc:
	 cargo doc --package pallet-deip  --open
