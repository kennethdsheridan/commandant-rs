.PHONY: all build build_linux build_macos build_wasm clean test fmt check lint help

# Set the binary name
BINARY_NAME := one_for_all

# Set the output directory
OUTPUT_DIR := ./target/release

# Determine the operating system
OS := $(shell uname -s)

# Set the output paths for Linux, macOS, and WASM binaries
BINARY_LINUX := $(OUTPUT_DIR)/linux/$(BINARY_NAME)
BINARY_MACOS := $(OUTPUT_DIR)/macos/$(BINARY_NAME)
BINARY_WASM := $(OUTPUT_DIR)/wasm/$(BINARY_NAME)

# Default target: build for the detected OS
all: build

# Build the binary for the detected OS
build:
ifeq ($(OS),Linux)
	@$(MAKE) build_linux
else ifeq ($(OS),Darwin)
	@$(MAKE) build_macos
else
	@echo "Unsupported OS"
endif

# Build the Linux binary
build_linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p $(BINARY_LINUX)
	cp ./target/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) $(BINARY_LINUX)

# Build the MacOS binary
build_macos:
	cargo build --release --target x86_64-apple-darwin
	mkdir -p $(BINARY_MACOS)
	cp ./target/x86_64-apple-darwin/release/$(BINARY_NAME) $(BINARY_MACOS)

# Build the WASM binary
build_wasm:
	cargo build --release --target wasm32-unknown-unknown
	mkdir -p $(OUTPUT_DIR)/wasm
	cp ./target/wasm32-unknown-unknown/release/$(BINARY_NAME) $(OUTPUT_DIR)/wasm/

# Clean the generated binaries and other build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Format the code
fmt:
	cargo fmt --all

# Check the code for errors without building
check:
	cargo check

# Run additional checks (clippy)
lint:
	cargo clippy -- -D warnings

# Show help text for this Makefile
help:
	@echo "Usage: make [TARGET]"
	@echo "Targets:"
	@echo "  all         Build the project for the detected OS"
	@echo "  build       Alias for 'all'"
	@echo "  build_linux Build the project for Linux"
	@echo "  build_macos Build the project for macOS"
	@echo "  build_wasm  Build the project for WebAssembly"
	@echo "  clean       Remove build artifacts"
	@echo "  test        Run tests"
	@echo "  fmt         Format the code"
	@echo "  check       Check the code for errors"
	@echo "  lint        Run clippy to catch common mistakes and improve your Rust code"