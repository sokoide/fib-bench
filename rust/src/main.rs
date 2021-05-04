extern crate stopwatch;
use stopwatch::Stopwatch;

fn fib(i: u64) -> u64 {
    if i <= 1 {
        return i;
    }
    fib(i - 1) + fib(i - 2)
}

fn main() {
    let mut sw = Stopwatch::new();

    for i in 40..44 {
        sw.reset();
        sw.start();
        let result = fib(i);
        sw.stop();
        println!("{}, fib({}) = {}", sw.elapsed_ms(), i, result);
    }
}
