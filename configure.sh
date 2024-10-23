#!/bin/bash

sudo dnf install \
    curl \
    binutils gcc \
    mingw32-binutils mingw32-gcc \
    mingw64-binutils mingw64-gcc \
    ucrt64-binutils

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

. "$HOME/.cargo/env"

rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
