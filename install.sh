#!/usr/bin/env bash

if [ ! -f "$(which cargo)" ]; then
    echo "Rust is not installed, please install it!"
    exit 1
fi

echo "Building..."
cargo build --release

echo "Installing..."
sudo mkdir -p /usr/local/share/rbxmidi

echo "Installing Library..."
sudo cp -r Library /usr/local/share/rbxmidi/

echo "Installing binary..."
sudo cp ./target/release/rbxmidi /usr/local/bin/rbxmidi

echo "Installing desktop entry..."
sudo cp RobloxMidi.desktop /usr/local/share/applications/

echo "!! Roblox MIDI is installed !!"