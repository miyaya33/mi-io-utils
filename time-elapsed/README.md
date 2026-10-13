# mi-time-elapsed

An attribute macro that prints the wall-clock execution time of functions.

## Usage

```toml
[dependencies]
mi-time-elapsed = "0.1"
```

```rust
use mi_time_elapsed::time_elapsed;

#[time_elapsed]
fn load_user(id: u64) -> Result<String, &'static str> {
    Ok(format!("user-{id}"))
}
```

When the function completes, it prints the function name and elapsed time.
It also prints when a function exits through an early `return`. Async
functions are supported too; in that case the timing is printed when the
future completes. The macro uses `std::time::Instant` and has no dependency
on Tokio, async-std, or any other async runtime.

## Limitations

- If an async future is dropped before completion, its timing is printed at
  drop time.

## License

MIT
