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

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Read a file as a line iterator, preserving line read errors.
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>> {
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
        assert_eq!(
            read_last_lines(&file, 10)?,
            vec!["one", "two", "three", "four"]
        );

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
}
