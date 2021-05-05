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

```bash
dotnet run -c release
```

* Java

```bash
javac Hoge.java
java Hoge
```

## Result on M1 Mac mini

* Times are all in seconds

| Fib | Py 3.9.4 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET6 preview3 | Open JDK 11.0.10 |
| --- | -------- | --------- | ----------- | --------- | -------------- | ---------------- |
| 40  | 20.3s    | 0.5s      | 0.3s        | 0.4s      | 0.5s           | 0.3s             |
| 41  | 32.8s    | 0.8s      | 0.5s        | 0.7s      | 0.7s           | 0.4s             |
| 42  | 53.2     | 1.4s      | 0.8s        | 1.1s      | 1.1s           | 0.7s             |
| 43  | 87.7s    | 2.2s      | 1.3s        | 1.8s      | 1.9s           | 1.2s             |
