# fib quick benchmark

## About

* Quick benchmark of non-optimized recursive `fib` function in different languages

## How to build

* rust

```bash
cargo build --release
```

* rust_wasm

```bash
cargo build --release --target wasm32-unknown-unknown
```

* rust_wasm_runner

```bash
cargo build --release
```

* go

```bash
go build  -ldflags "-s -w"
```

* .NET

```
dotnet run -c release
```

## Result on M1 Mac mini

* Times are all in seconds

| Fib | Py 3.9.4 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET6 preview3 |
| --- | -------- | --------- | ----------- | --------- | -------------- |
| 40  | 20.3s    | 0.5s      | 0.3s        | 0.4s      | 0.5s           |
| 41  | 32.8s    | 0.8s      | 0.5s        | 0.7s      | 0.7s           |
| 42  | 53.2     | 1.4s      | 0.8s        | 1.1s      | 1.1s           |
| 43  | 87.7s    | 2.2s      | 1.3s        | 1.8s      | 1.9s           |

