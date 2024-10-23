#!/bin/bash

rustup target add --toolchain x86_64-pc-windows-gnu
rustup target add --toolchain x86_64-unknown-linux-gnu
rustup target add --toolchain armhf-unknown-linux-gnu

cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target armhf-unknown-linux-gnu