# mi-io-utils

Rust IO utility crates collected in one workspace.

## Crates

| Crate | Description | Version |
|-------|-------------|---------|
| [mi-simple-io](./simple-io) | Synchronous IO utilities | [![Crates.io](https://img.shields.io/crates/v/mi-simple-io.svg)](https://crates.io/crates/mi-simple-io) |

## Current Crate

A simple and practical IO utility library for Rust.

```toml
[dependencies]
mi-simple-io = "0.1.0"
```

```rust
use mi_simple_io::{append_file, read_file, write_file};

fn main() -> std::io::Result<()> {
    write_file("hello.txt", "Hello")?;
    append_file("hello.txt", ", Rust!")?;
    assert_eq!(read_file("hello.txt")?, "Hello, Rust!");
    Ok(())
}
```

## Planned Crates

- `simple-io/`: synchronous file, directory, temp file, input, and stats helpers.
- `async-io/`: future async IO utilities.
- `io-extras/`: future higher-level IO extensions.
- `io-macros/`: future procedural macros if they become useful.

Only `simple-io/` is currently implemented.

## Local Verification

```bash
cargo fmt --all -- --check
cargo test -p mi-simple-io
cargo test -p mi-simple-io --doc
cargo run -p mi-simple-io --example quick_start
cargo package -p mi-simple-io --allow-dirty
```
