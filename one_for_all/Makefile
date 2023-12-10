.PHONY: all build build_linux build_macos clean test fmt check lint help

# Set the binary name
BINARY_NAME := one_for_all

# Determine the operating system
OS := $(shell uname -s)
ifeq ($(OS),Linux)
    BINARY_EXT := linux
    TARGET_ARCH := x86_64-unknown-linux-gnu
else
    BINARY_EXT := darwin
    TARGET_ARCH := x86_64-apple-darwin
endif

# Set the output directory
OUTPUT_DIR := ./oneforall/one_for_all/target/linux/release

# Set the output paths for Linux and macOS binaries
BINARY_LINUX := $(OUTPUT_DIR)/$(BINARY_NAME)
BINARY_MACOS := $(OUTPUT_DIR)/$(BINARY_NAME)_macos

# Default target: build for the current OS
all: build

# Build the binary for the current OS
build:
	cargo build --release
	mkdir -p $(OUTPUT_DIR)
	if [ -f $(OUTPUT_DIR)/$(BINARY_NAME) ]; then \
		cp $(OUTPUT_DIR)/$(BINARY_NAME) $(OUTPUT_DIR)/$(BINARY_NAME)_$(BINARY_EXT); \
	fi

# Build the Linux binary
build_linux:
	cargo build --release --target $(TARGET_ARCH)
	mkdir -p $(OUTPUT_DIR)
	cp ./target/$(TARGET_ARCH)/release/$(BINARY_NAME) $(BINARY_LINUX)

# Build the MacOS binary
build_macos:
	cargo build --release --target $(TARGET_ARCH)
	mkdir -p $(OUTPUT_DIR)
	cp $(OUTPUT_DIR)/$(BINARY_NAME) $(BINARY_MACOS)

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
	@echo "  all         Build the project for the current OS"
	@echo "  build       Alias for 'all'"
	@echo "  build_linux Build the project for Linux"
	@echo "  build_macos Build the project for macOS"
	@echo "  clean       Remove build artifacts"
	@echo "  test        Run tests"
	@echo "  fmt         Format the code"
	@echo "  check       Check the code for errors"
	@echo "  lint        Run clippy to catch common mistakes and improve your Rust code"
