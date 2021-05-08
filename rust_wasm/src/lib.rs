#[no_mangle]
pub extern "C" fn fib(i: u64) -> u64 {
    match i {
        0 => 0,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    }
}
