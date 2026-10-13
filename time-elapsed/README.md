# mi-time-elapsed

An attribute macro that prints the wall-clock execution time of async
functions.

## Usage

```toml
[dependencies]
mi-time-elapsed = "0.1"
```

```rust
use mi_time_elapsed::time_elapsed;

#[time_elapsed]
async fn load_user(id: u64) -> Result<String, &'static str> {
    Ok(format!("user-{id}"))
}
```

When the function completes, it prints the function signature and elapsed
time. The macro uses `std::time::Instant` and has no dependency on Tokio,
async-std, or any other async runtime.

## Limitations

- `#[time_elapsed]` only accepts async functions.
- Timing is printed when the future completes. If the future is dropped before
  completion, no timing line is printed.

## License

MIT
