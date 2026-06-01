# mi-simple-io Design

## Goal

Build the first public crate in the `my-io-utils` monorepo: a small synchronous IO utility library located in `simple-io/` and published as `mi-simple-io`.

The current `t.rs` file is only a migration reference. Its useful API ideas will be moved into `simple-io/src/lib.rs`, then `t.rs` will be deleted before completion.

## Repository Shape

The repository is a workspace for related IO crates:

- `simple-io/`: current basic synchronous IO crate.
- `async-io/`: future async IO crate, not created yet.
- `io-extras/`: future extension crate, not created yet.
- `io-macros/`: future proc-macro crate, not created yet.

For this implementation, only `simple-io` is a workspace member. The future crates will be mentioned in README as planned modules, but empty crates will not be created.

The root `Cargo.toml` should be a workspace manifest only. It should not define a package target.

## Crate Identity

Directory: `simple-io/`

Published package name: `mi-simple-io`

Rust import name:

```rust
use mi_simple_io::*;
```

The crate should include publishing metadata: description, license, repository, README, keywords, categories, and workspace-inherited version, edition, authors, license, and repository where appropriate.

## Public API

The crate exposes simple synchronous helpers based on `std`:

- File read/write:
  - `read_file`
  - `read_bytes`
  - `write_file`
  - `append_file`
  - `write_or_append`
- File and path checks:
  - `exists`
  - `is_file`
  - `is_dir`
  - `file_size`
- Directory operations:
  - `create_dir`
  - `remove`
  - `clear_dir`
- Line reading:
  - `read_lines`
  - `read_all_lines`
  - `read_first_lines`
  - `read_last_lines`
- Copy and move:
  - `copy_file`
  - `move_file`
- Path helpers:
  - `file_stem`
  - `file_extension`
  - `current_dir`
  - `temp_dir`
- Temporary files:
  - `TempFile::new`
  - `TempFile::with_content`
  - `TempFile::path`
  - `TempFile::read`
- User input:
  - `read_input`
  - `confirm`
- Safe write:
  - `safe_write`
- Stats:
  - `FileStats`
  - `file_stats`

`read_password` should not be part of the default API. If included, it must be behind an optional `password` feature using `rpassword`.

## Behavior Notes

All fallible operations return `std::io::Result`.

`read_lines` should preserve IO errors while reading lines instead of silently dropping them. A return type such as `io::Result<impl Iterator<Item = io::Result<String>>>` is acceptable.

`TempFile` should avoid deterministic filename collisions. It should create files under the system temp directory and remove the file on drop.

`safe_write` should write to a sibling temporary path and then rename it into place. If writing fails, it should attempt to clean up the temporary path.

`remove` removes files and directories. Missing paths should return the same error behavior as the underlying `std::fs` calls.

## Examples And Docs

Add a quick-start example:

- `simple-io/examples/quick_start.rs`

The example should demonstrate:

- writing and reading a file
- appending content
- reading lines
- using `TempFile`
- getting `FileStats`
- cleaning up created files

Add crate-level docs and short doctests for core functions where they are useful. Doctests must avoid leaving files in the repository.

Update README with:

- monorepo purpose
- current crate status
- install snippet for `mi-simple-io`
- short usage example
- planned future crates
- local verification commands

## Tests

Tests should be isolated from the repository working directory by creating unique temporary directories or temporary files under the system temp directory.

Cover at least:

- write, read, append, and write-or-append
- byte reads and file size
- exists, is-file, and is-dir
- create, clear, and remove directory
- line reading, first N lines, and last N lines
- copy and move
- file stem and extension behavior
- `TempFile` content and cleanup behavior
- `safe_write`
- `file_stats`

## Verification

Before completion, run:

```bash
cargo fmt --all -- --check
cargo test -p mi-simple-io
cargo test -p mi-simple-io --doc
cargo run -p mi-simple-io --example quick_start
cargo package -p mi-simple-io --allow-dirty
```

Use `--allow-dirty` only because this is local pre-publication validation inside an active working tree.

## Out Of Scope

- Implementing `async-io`, `io-extras`, or `io-macros`.
- Publishing to crates.io.
- Adding async APIs.
- Adding broad external dependencies.
- Preserving `t.rs`.
