extern crate stopwatch;
use stopwatch::Stopwatch;

use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(
        &engine,
        "../rust_wasm/target/wasm32-unknown-unknown/release/rust_wasm.wasm",
    )?;

    let instance = Instance::new(&mut store, &module, &[])?;
    let fib = instance.get_typed_func::<u64, u64>(&mut store, "fib")?;

    let mut sw = Stopwatch::new();

    for i in 40..44u64 {
        sw.reset();
        sw.start();
        let result = fib.call(&mut store, i)?;
        sw.stop();
        println!("{}, fib({}) = {}", sw.elapsed_ms(), i, result);
    }

    Ok(())
}
