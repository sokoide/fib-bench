#[no_mangle]
pub extern "C" fn fib(i: u64) -> u64 {
    if i <= 1 {
        return i;
    }
    fib(i - 1) + fib(i - 2)
}
