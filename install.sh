#!/bin/bash

# Check if the user is root
if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

# Create variable for the app name
APP_NAME="OneForAll"

# Change to the directory where the script is located
cd "$(dirname "$0")"

# Update and install necessary packages (optional, based on your program requirements)
apt-get update
apt-get install -y pkg-config curl git build-essential

# Check if Rust is installed, install if it's not
if ! command -v rustc &>/dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Build the Rust program in release mode
echo "Building the Rust program..."
cargo build --release

# Copy the binary to a system-wide location and use the APP_NAME variable
echo "Copying the binary to /usr/local/bin..."
cp target/release/$APP_NAME /usr/local/bin




