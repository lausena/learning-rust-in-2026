# build-script-example

A Rust project demonstrating C interoperability via FFI (Foreign Function Interface). The main binary calls a C function (`count_spaces`) from Rust using a Cargo build script to compile and link the C code.

## Prerequisites

- Rust (edition 2024)
- A C compiler (cc)

## Building and Running

```sh
cargo build
cargo run
```

## How It Works

1. `build.rs` uses the [`cc`](https://crates.io/crates/cc) crate to compile `src/count_spaces.c` into a static library.
2. `src/main.rs` declares the C function with `unsafe extern "C"` and calls it through `libc` using a safe Rust wrapper.

## Project Structure

- `src/count_spaces.c` — C function that counts the number of spaces in a string
- `src/main.rs` — Rust entry point; wraps the FFI call in a safe `call_count_spaces` function
- `build.rs` — Cargo build script that compiles the C source and links it into the binary
