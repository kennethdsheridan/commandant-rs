.PHONY: all build_linux build_macos clean test fmt check

# Set the binary name and source directory
BINARY_NAME := one-for-all
SOURCES_DIR := ./

# Determine the operating system
ifeq ($(shell uname -s),Linux)
    BINARY_EXT := linux
else
    BINARY_EXT := darwin
endif

# Set the output directory
OUTPUT_DIR := ./target/release

# Set the output paths for Linux and macOS binaries
BINARY_LINUX := $(OUTPUT_DIR)/$(BINARY_NAME)_linux
BINARY_MACOS := $(OUTPUT_DIR)/$(BINARY_NAME)_macos

# Default target: build for the current OS
all: build

# Build the binary for the current OS
build:
	cargo build --release
	cp $(OUTPUT_DIR)/$(BINARY_NAME) $(OUTPUT_DIR)/$(BINARY_NAME)_$(BINARY_EXT)

# Build the Linux binary
build_linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	cp $(OUTPUT_DIR)/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) $(BINARY_LINUX)

# Build the MacOS binary
build_macos:
	cargo build --release --target x86_64-apple-darwin
	cp $(OUTPUT_DIR)/x86_64-apple-darwin/release/$(BINARY_NAME) $(BINARY_MACOS)

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
	@echo "  all        Build the project for the current OS"
	@echo "  build      Alias for 'all'"
	@echo "  build_linux Build the project for Linux"
	@echo "  build_macos Build the project for macOS"
	@echo "  clean      Remove build artifacts"
	@echo "  test       Run tests"
	@echo "  fmt        Format the code"
	@echo "  check      Check the code for errors"
	@echo "  lint       Run clippy to catch common mistakes and improve your Rust code"
