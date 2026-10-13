use futures::executor::block_on;
use mi_time_elapsed::time_elapsed;

#[time_elapsed]
fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[time_elapsed]
fn may_return_early(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    value
}

#[time_elapsed]
async fn fallible() -> Result<&'static str, &'static str> {
    Ok("done")
}

#[test]
fn async_function_keeps_its_result() {
    assert_eq!(add(20, 22), 42);
    assert_eq!(may_return_early(-5), 0);
    assert_eq!(block_on(fallible()), Ok("done"));
}
