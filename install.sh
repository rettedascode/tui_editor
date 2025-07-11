#!/bin/bash
set -e

echo "Checking for Rust..."
if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

echo "Building tui_editor in release mode..."
cargo build --release

# Optionally copy to ~/.cargo/bin
TARGET="$HOME/.cargo/bin/tui_editor"
cp ./target/release/tui_editor "$TARGET"
echo "Installed tui_editor to $TARGET"
echo "You may need to add $HOME/.cargo/bin to your PATH if it's not already."
echo "You can now run tui_editor from any terminal." 