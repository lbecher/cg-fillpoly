#!/bin/bash

sudo dnf install \
    curl binutils \
    mingw32-binutils mingw32-gcc \
    mingw64-binutils mingw64-gcc \
    ucrt64-binutils

rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
