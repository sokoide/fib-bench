extern crate stopwatch;
use stopwatch::Stopwatch;

use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let store = Store::new(&engine);

    let module = Module::from_file(
        &engine,
        "../rust_wasm/target/wasm32-unknown-unknown/release/rust_wasm.wasm",
    )?;

    let instance = Instance::new(&store, &module, &[])?;

    let fib = instance
        .get_typed_func::<u64, u64>("fib")
        .expect("`fib` was not an exported function");

    let mut sw = Stopwatch::new();

    for i in 40..44 {
        sw.reset();
        sw.start();
        let result = fib.call(i)?;
        sw.stop();
        println!("{}, fib({}) = {}", sw.elapsed_ms(), i, result);
    }

    Ok(())
}
