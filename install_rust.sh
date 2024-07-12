#!/bin/bash

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if wget is installed
if ! command_exists wget; then
    echo "Error: wget is not installed. Please install wget and try again."
    exit 1
fi

# Determine the OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Check if the OS is supported
if [ "$OS" != "linux" ] && [ "$OS" != "darwin" ]; then
    echo "Error: This script only supports Linux and macOS."
    exit 1
fi

# Download and run the Rust installer
if ! wget https://sh.rustup.rs -O rustup-init.sh; then
    echo "Failed to download Rust installer"
    exit 1
fi

if ! sh rustup-init.sh -y; then
    echo "Rust installation failed"
    exit 1
fi

echo "Rust and Cargo have been successfully installed!"

# Clean up
rm rustup-init.sh

# Remind the user to source the new PATH
echo "Please run 'source \$HOME/.cargo/env' or restart your shell to update your PATH."
