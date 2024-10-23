#!/bin/bash

cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-gnu

mkdir bin -p

cp ./target/x86_64-pc-windows-gnu/release/cg-fillpoly.exe ./bin
cp ./target/x86_64-unknown-linux-gnu/release/cg-fillpoly ./bin