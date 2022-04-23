#!/usr/bin/env bash

echo "Building..."
cargo b --release
echo "Installing..."
mkdir -p $HOME/.local/share/bin
mkdir -p $HOME/.local/share/applications
cp target/release/rbxmidi $HOME/.local/share/bin/rbxmidi
cp rbxmidi.desktop $HOME/.local/share/applications/rbxmidi.desktop
echo "Done."
