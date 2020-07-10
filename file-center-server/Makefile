SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc
UPX := $(shell command -v upx 2> /dev/null)

.PHONY: all
all:
	cargo build --release 
	strip target/release/file-center
ifdef UPX
		upx target/release/file-center 
endif
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/file-center
	upx target/release/file-center

.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release
	strip target/x86_64-unknown-linux-musl/release/file-center
	upx target/x86_64-unknown-linux-musl/release/file-center

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release
	strip target/x86_64-pc-windows-gnu/release/file-center.exe
	upx target/x86_64-pc-windows-gnu/release/file-center.exe

.PHONY: build-mac
build-mac:
	cargo build --target x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/file-center
	upx target/x86_64-apple-darwin/release/file-center

.PHONY: run
run:
ifndef ARGS
	@echo Run "make run" with ARGS set to pass arguments…
endif
	cargo run --release -- $(ARGS)

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean