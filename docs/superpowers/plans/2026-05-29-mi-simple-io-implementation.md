# mi-simple-io Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and verify the `mi-simple-io` public crate in the `simple-io/` workspace member, then delete the temporary `t.rs` reference file.

**Architecture:** The repository root is a workspace-only manifest. `simple-io/` contains one focused sync IO library with standard-library-first APIs, isolated tests, docs, and a quick-start example. Future crates are documented as planned workspace modules but are not created yet.

**Tech Stack:** Rust 2021, Cargo workspace, `std::fs`, `std::io`, optional `rpassword` feature, Rust unit tests, doc tests, Cargo examples.

---

## File Structure

- Modify `Cargo.toml`: make it a workspace-only manifest and share package metadata.
- Modify `simple-io/Cargo.toml`: publishable crate metadata for package `mi-simple-io`, optional `password` feature.
- Modify `simple-io/src/lib.rs`: public sync IO API, crate docs, unit tests.
- Create `simple-io/examples/quick_start.rs`: executable example.
- Modify `README.md`: monorepo overview, install snippet, usage, future modules, verification commands.
- Delete `t.rs`: migration reference should not remain.

## Task 1: Workspace And Crate Metadata

**Files:**
- Modify: `Cargo.toml`
- Modify: `simple-io/Cargo.toml`

- [ ] **Step 1: Write workspace and crate manifests**

Set root `Cargo.toml` to:

```toml
[workspace]
members = ["simple-io"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["mi"]
license = "MIT"
repository = "https://github.com/miyaya33/mi-io-utils"
homepage = "https://github.com/miyaya33/mi-io-utils"
documentation = "https://docs.rs/mi-simple-io"
readme = "README.md"
```

Set `simple-io/Cargo.toml` to:

```toml
[package]
name = "mi-simple-io"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
description = "Small synchronous IO utilities for common file and terminal workflows."
readme = "../README.md"
keywords = ["io", "file", "filesystem", "utility"]
categories = ["filesystem", "command-line-utilities"]

[features]
default = []
password = ["dep:rpassword"]

[dependencies]
rpassword = { version = "7.4", optional = true }
```

- [ ] **Step 2: Verify metadata parses**

Run: `cargo metadata --no-deps --format-version 1`

Expected: succeeds and lists package name `mi-simple-io`.

## Task 2: Core File IO Tests And Implementation

**Files:**
- Modify: `simple-io/src/lib.rs`

- [ ] **Step 1: Write failing tests for file read/write APIs**

Create `simple-io/src/lib.rs` with crate docs, empty API surface, and these tests:

```rust
//! # mi-simple-io
//!
//! Small synchronous IO utilities for common file and terminal workflows.

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> io::Result<PathBuf> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mi_simple_io_{}_{}_{}",
            name,
            std::process::id(),
            unique
        ));
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    #[test]
    fn write_read_append_and_write_or_append_text() -> io::Result<()> {
        let dir = test_dir("text")?;
        let file = dir.join("notes.txt");

        write_file(&file, "hello")?;
        assert_eq!(read_file(&file)?, "hello");

        append_file(&file, " world")?;
        assert_eq!(read_file(&file)?, "hello world");

        let new_file = dir.join("new.txt");
        write_or_append(&new_file, "first")?;
        write_or_append(&new_file, " second")?;
        assert_eq!(read_file(&new_file)?, "first second");

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn reads_bytes_and_reports_file_size() -> io::Result<()> {
        let dir = test_dir("bytes")?;
        let file = dir.join("data.bin");

        fs::write(&file, [0_u8, 1, 2, 3])?;

        assert_eq!(read_bytes(&file)?, vec![0, 1, 2, 3]);
        assert_eq!(file_size(&file)?, 4);

        fs::remove_dir_all(dir)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p mi-simple-io write_read_append_and_write_or_append_text reads_bytes_and_reports_file_size`

Expected: FAIL with unresolved functions such as `write_file`, `read_file`, `append_file`, `write_or_append`, `read_bytes`, and `file_size`.

- [ ] **Step 3: Implement minimal file IO APIs**

Add above the test module:

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Read a UTF-8 text file into a string.
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Read a file into bytes.
pub fn read_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// Write text to a file, replacing existing content.
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Append text to a file, creating it when missing.
pub fn append_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(content.as_bytes())
}

/// Write text when the file is missing, otherwise append it.
pub fn write_or_append<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    if path.as_ref().exists() {
        append_file(path, content)
    } else {
        write_file(path, content)
    }
}

/// Return file size in bytes.
pub fn file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p mi-simple-io write_read_append_and_write_or_append_text reads_bytes_and_reports_file_size`

Expected: PASS.

## Task 3: Directory And Path APIs

**Files:**
- Modify: `simple-io/src/lib.rs`

- [ ] **Step 1: Add failing tests for existence, directory, remove, copy, move, and path helpers**

Append these tests inside `mod tests`:

```rust
    #[test]
    fn checks_file_and_directory_state() -> io::Result<()> {
        let dir = test_dir("state")?;
        let file = dir.join("state.txt");

        assert!(exists(&dir));
        assert!(is_dir(&dir));
        assert!(!is_file(&dir));

        write_file(&file, "state")?;
        assert!(exists(&file));
        assert!(is_file(&file));
        assert!(!is_dir(&file));

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn creates_clears_and_removes_directories() -> io::Result<()> {
        let dir = test_dir("dirs")?;
        let nested = dir.join("a/b");
        let file = nested.join("file.txt");

        create_dir(&nested)?;
        write_file(&file, "content")?;
        assert!(is_file(&file));

        clear_dir(dir.join("a"))?;
        assert!(is_dir(dir.join("a")));
        assert!(!exists(&file));

        remove(dir.join("a"))?;
        assert!(!exists(dir.join("a")));

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn copies_moves_and_extracts_path_parts() -> io::Result<()> {
        let dir = test_dir("copy_move")?;
        let source = dir.join("Report.TXT");
        let copied = dir.join("copied.txt");
        let moved = dir.join("moved.md");

        write_file(&source, "copy me")?;
        assert_eq!(copy_file(&source, &copied)?, 7);
        assert_eq!(read_file(&copied)?, "copy me");

        move_file(&copied, &moved)?;
        assert!(!exists(&copied));
        assert_eq!(read_file(&moved)?, "copy me");

        assert_eq!(file_stem(&source), "Report");
        assert_eq!(file_extension(&source), "txt");
        assert!(current_dir()?.is_dir());
        assert!(temp_dir().is_dir());

        fs::remove_dir_all(dir)?;
        Ok(())
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p mi-simple-io checks_file_and_directory_state creates_clears_and_removes_directories copies_moves_and_extracts_path_parts`

Expected: FAIL with unresolved functions such as `exists`, `is_dir`, `is_file`, `create_dir`, `clear_dir`, `remove`, `copy_file`, `move_file`, `file_stem`, `file_extension`, `current_dir`, and `temp_dir`.

- [ ] **Step 3: Implement directory and path APIs**

Extend imports:

```rust
use std::path::{Path, PathBuf};
```

Add API functions above the test module:

```rust
/// Return whether a path exists.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Return whether a path is a regular file.
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Return whether a path is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Create a directory and any missing parents.
pub fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Remove a file or directory tree.
pub fn remove<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// Remove all directory contents while keeping the directory itself.
pub fn clear_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        remove(entry.path())?;
    }
    Ok(())
}

/// Copy a file and return the number of bytes copied.
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    fs::copy(from, to)
}

/// Move or rename a file or directory.
pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    fs::rename(from, to)
}

/// Return the file stem as UTF-8 lossy text, or an empty string.
pub fn file_stem<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Return the lowercase file extension as UTF-8 lossy text, or an empty string.
pub fn file_extension<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
}

/// Return the current working directory.
pub fn current_dir() -> io::Result<PathBuf> {
    std::env::current_dir()
}

/// Return the system temporary directory.
pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p mi-simple-io checks_file_and_directory_state creates_clears_and_removes_directories copies_moves_and_extracts_path_parts`

Expected: PASS.

## Task 4: Line Reading, Stats, Safe Write, And TempFile

**Files:**
- Modify: `simple-io/src/lib.rs`

- [ ] **Step 1: Add failing tests for line APIs, stats, safe write, and TempFile**

Append these tests inside `mod tests`:

```rust
    #[test]
    fn reads_lines_and_line_ranges() -> io::Result<()> {
        let dir = test_dir("lines")?;
        let file = dir.join("lines.txt");
        write_file(&file, "one\ntwo\nthree\nfour\n")?;

        let lines = read_all_lines(&file)?;
        assert_eq!(lines, vec!["one", "two", "three", "four"]);

        let streamed = read_lines(&file)?.collect::<io::Result<Vec<_>>>()?;
        assert_eq!(streamed, vec!["one", "two", "three", "four"]);

        assert_eq!(read_first_lines(&file, 2)?, vec!["one", "two"]);
        assert_eq!(read_last_lines(&file, 2)?, vec!["three", "four"]);
        assert_eq!(read_last_lines(&file, 10)?, vec!["one", "two", "three", "four"]);

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn safely_writes_and_reports_file_stats() -> io::Result<()> {
        let dir = test_dir("safe_stats")?;
        let file = dir.join("stats.txt");

        safe_write(&file, "hello rust\nsecond line")?;
        assert_eq!(read_file(&file)?, "hello rust\nsecond line");
        assert!(!exists(file.with_extension("tmp")));

        let stats = file_stats(&file)?;
        assert_eq!(stats.size, 22);
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 4);
        assert_eq!(stats.chars, 22);

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn temp_file_writes_reads_and_removes_on_drop() -> io::Result<()> {
        let path;
        {
            let temp = TempFile::with_content("unit", "temporary data")?;
            path = temp.path().to_path_buf();
            assert!(is_file(&path));
            assert_eq!(temp.read()?, "temporary data");
        }
        assert!(!exists(path));
        Ok(())
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p mi-simple-io reads_lines_and_line_ranges safely_writes_and_reports_file_stats temp_file_writes_reads_and_removes_on_drop`

Expected: FAIL with unresolved items such as `read_all_lines`, `read_lines`, `read_first_lines`, `read_last_lines`, `safe_write`, `file_stats`, `FileStats`, and `TempFile`.

- [ ] **Step 3: Implement line APIs, stats, safe_write, and TempFile**

Extend imports:

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
```

Add API items above the test module:

```rust
/// Read a file as a line iterator, preserving line read errors.
pub fn read_lines<P: AsRef<Path>>(
    path: P,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

/// Read all lines from a file.
pub fn read_all_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    read_lines(path)?.collect()
}

/// Read the first `n` lines from a file.
pub fn read_first_lines<P: AsRef<Path>>(path: P, n: usize) -> io::Result<Vec<String>> {
    read_lines(path)?.take(n).collect()
}

/// Read the last `n` lines from a file.
pub fn read_last_lines<P: AsRef<Path>>(path: P, n: usize) -> io::Result<Vec<String>> {
    let lines = read_all_lines(path)?;
    let start = lines.len().saturating_sub(n);
    Ok(lines[start..].to_vec())
}

/// Write text via a sibling temporary file before replacing the destination.
pub fn safe_write<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let path = path.as_ref();
    let temp_path = path.with_extension("tmp");

    if let Err(error) = write_file(&temp_path, content) {
        let _ = fs::remove_file(&temp_path);
        return Err(error);
    }

    if let Err(error) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(error);
    }

    Ok(())
}

/// Basic text file statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStats {
    pub size: u64,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

/// Read a text file and return basic statistics.
pub fn file_stats<P: AsRef<Path>>(path: P) -> io::Result<FileStats> {
    let content = read_file(path)?;
    Ok(FileStats {
        size: content.len() as u64,
        lines: content.lines().count(),
        words: content.split_whitespace().count(),
        chars: content.chars().count(),
    })
}

/// Temporary file removed when dropped.
#[derive(Debug)]
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Create a new empty temporary file with a prefix.
    pub fn new(prefix: &str) -> io::Result<Self> {
        for attempt in 0..100 {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_nanos();
            let path = temp_dir().join(format!(
                "{}_{}_{}_{}",
                prefix,
                std::process::id(),
                unique,
                attempt
            ));

            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(_) => return Ok(Self { path }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }

        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "could not create a unique temporary file",
        ))
    }

    /// Create a temporary file with content.
    pub fn with_content(prefix: &str, content: &str) -> io::Result<Self> {
        let temp = Self::new(prefix)?;
        write_file(&temp.path, content)?;
        Ok(temp)
    }

    /// Return the temporary file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Read the temporary file as text.
    pub fn read(&self) -> io::Result<String> {
        read_file(&self.path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p mi-simple-io reads_lines_and_line_ranges safely_writes_and_reports_file_stats temp_file_writes_reads_and_removes_on_drop`

Expected: PASS.

## Task 5: User Input And Optional Password API

**Files:**
- Modify: `simple-io/src/lib.rs`

- [ ] **Step 1: Add compile-time tests for input-facing APIs**

Append this test inside `mod tests`:

```rust
    #[test]
    fn confirm_defaults_match_empty_or_unknown_answers() {
        assert_eq!(parse_confirmation("", true), true);
        assert_eq!(parse_confirmation("", false), false);
        assert_eq!(parse_confirmation("maybe", true), true);
        assert_eq!(parse_confirmation("maybe", false), false);
        assert_eq!(parse_confirmation("yes", false), true);
        assert_eq!(parse_confirmation("Y", false), true);
        assert_eq!(parse_confirmation("no", true), false);
        assert_eq!(parse_confirmation("N", true), false);
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p mi-simple-io confirm_defaults_match_empty_or_unknown_answers`

Expected: FAIL with unresolved function `parse_confirmation`.

- [ ] **Step 3: Implement input APIs and optional password API**

Add above the test module:

```rust
/// Read a trimmed line from standard input after printing a prompt.
pub fn read_input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Parse confirmation text using a default for empty or unknown input.
pub fn parse_confirmation(input: &str, default: bool) -> bool {
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => default,
    }
}

/// Ask a yes/no question and return the selected answer.
pub fn confirm(prompt: &str, default: bool) -> bool {
    let default_text = if default { "Y/n" } else { "y/N" };
    let input = read_input(&format!("{prompt} [{default_text}]: ")).unwrap_or_default();
    parse_confirmation(&input, default)
}

/// Read a password from the terminal without echoing input.
#[cfg(feature = "password")]
pub fn read_password(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    rpassword::read_password()
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p mi-simple-io confirm_defaults_match_empty_or_unknown_answers`

Expected: PASS.

- [ ] **Step 5: Verify optional password feature compiles**

Run: `cargo check -p mi-simple-io --features password`

Expected: PASS.

## Task 6: Docs, Example, README, And Delete t.rs

**Files:**
- Modify: `simple-io/src/lib.rs`
- Create: `simple-io/examples/quick_start.rs`
- Modify: `README.md`
- Delete: `t.rs`

- [ ] **Step 1: Add crate docs with a doctest**

Update the top of `simple-io/src/lib.rs` to:

```rust
//! # mi-simple-io
//!
//! Small synchronous IO utilities for common file and terminal workflows.
//!
//! ```
//! use mi_simple_io::{read_file, remove, write_file};
//!
//! # fn main() -> std::io::Result<()> {
//! let file = std::env::temp_dir().join("mi_simple_io_doctest.txt");
//! write_file(&file, "hello")?;
//! assert_eq!(read_file(&file)?, "hello");
//! remove(&file)?;
//! # Ok(())
//! # }
//! ```
```

- [ ] **Step 2: Create quick-start example**

Create `simple-io/examples/quick_start.rs`:

```rust
use mi_simple_io::{
    append_file, create_dir, file_stats, read_all_lines, read_file, remove, write_file, TempFile,
};

fn main() -> std::io::Result<()> {
    let root = std::env::temp_dir().join("mi_simple_io_quick_start");
    if root.exists() {
        remove(&root)?;
    }
    create_dir(&root)?;

    let log = root.join("hello.txt");
    write_file(&log, "Hello, Rust!\n")?;
    append_file(&log, "Simple IO makes common tasks short.\n")?;

    println!("{}", read_file(&log)?);
    for line in read_all_lines(&log)? {
        println!("line: {line}");
    }

    let temp = TempFile::with_content("quick_start", "temporary data")?;
    println!("temp: {}", temp.path().display());

    let stats = file_stats(&log)?;
    println!(
        "size={} lines={} words={} chars={}",
        stats.size, stats.lines, stats.words, stats.chars
    );

    remove(root)?;
    Ok(())
}
```

- [ ] **Step 3: Update README**

Replace `README.md` with:

```markdown
# my-io-utils

Rust IO utility crates collected in one workspace.

## Current Crate

`simple-io/` is the first published crate in this repository.

```toml
[dependencies]
mi-simple-io = "0.1"
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
```

- [ ] **Step 4: Delete migration reference**

Delete `t.rs`.

- [ ] **Step 5: Run docs and example checks**

Run:

```bash
cargo test -p mi-simple-io --doc
cargo run -p mi-simple-io --example quick_start
```

Expected: both PASS.

## Task 7: Final Formatting And Package Verification

**Files:**
- Verify all modified files

- [ ] **Step 1: Format the workspace**

Run: `cargo fmt --all`

Expected: command succeeds.

- [ ] **Step 2: Check formatting**

Run: `cargo fmt --all -- --check`

Expected: PASS.

- [ ] **Step 3: Run full crate tests**

Run: `cargo test -p mi-simple-io`

Expected: PASS.

- [ ] **Step 4: Run doc tests**

Run: `cargo test -p mi-simple-io --doc`

Expected: PASS.

- [ ] **Step 5: Run example**

Run: `cargo run -p mi-simple-io --example quick_start`

Expected: PASS and prints the example text, line output, temp path, and stats.

- [ ] **Step 6: Verify package build**

Run: `cargo package -p mi-simple-io --allow-dirty`

Expected: PASS and creates a package for `mi-simple-io`.

- [ ] **Step 7: Inspect final working tree**

Run: `git status --short`

Expected: changed files include workspace metadata, `simple-io`, README, docs, and deletion of `t.rs`. `.idea/` may remain untracked if it was already present and should not be added.
