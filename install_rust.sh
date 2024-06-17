#!/bin/bash

# Define the installation directory for Rust and Cargo
export RUSTUP_HOME=/opt/rustup
export CARGO_HOME=/opt/cargo

# Install Rust and Cargo using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's bin directory to the PATH environment variable
echo 'export PATH=$PATH:/opt/cargo/bin' >> ~/.bashrc
source ~/.bashrc

# Verify the installation
rustc --version
cargo --version

echo "Rust and Cargo have been installed successfully."
