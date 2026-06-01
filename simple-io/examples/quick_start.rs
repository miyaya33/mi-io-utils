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
    // Print file path
    println!("File path: {}", log.display());
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

    // remove(root)?;
    Ok(())
}
