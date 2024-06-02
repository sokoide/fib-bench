# fib quick benchmark

## About

* Quick benchmark of non-optimized recursive `fib` function in different languages

## How to build

* rust

```bash
cd rust
make run
```

* rust_wasm

```bash
rustup target add wasm32-unknown-unknown
cd rust_wasm
make
```

* rust_wasm_runner

```bash
cd rust_wasm_runner
make run
```

* go

```bash
cd go
make run
```

* .NET

```bash
cd net
make run # .net8
make run6 # .net6
```

* Java

```bash
cd java
make run
```

* C

```
cd c
make run
```

## Result on M1 Mac mini

### 2024 June

* Rust Wasm used wasmtime 21.0.1
* .NET SDK 8.0.301 is used for .NET 8.0.6 runtime
* .NET SDK 6.0.131 is used for .NET 6.0.31 runtime

| Fib | Py 3.12.3 | Go 1.22.3 | Rust 1.78.0 | Rust Wasm | .NET 8.0.6 | .NET 6.0.31 | Open JDK 21.0.3 | C (clang 18.1.6) |
| --- | --------- | --------- | ----------- | --------- | ---------- | ----------- | --------------- | ---------------- |
| 40  | 10.8s     | 0.3s      | 0.3s        | 0.4s      | 0.3s       | 0.5s        | 0.3s            | 0.3s             |
| 41  | 17.4s     | 0.6s      | 0.5s        | 0.6s      | 0.5s       | 0.8s        | 0.5s            | 0.5s             |
| 42  | 28.1s     | 1.0s      | 0.8s        | 1.0s      | 0.7s       | 1.3s        | 0.8s            | 0.8s             |
| 43  | 45.4s     | 1.5s      | 1.4s        | 1.6s      | 1.3s       | 2.0s        | 1.3s            | 1.3s             |

### 2021 May

* Times are all in seconds
* clang 11.1.0 and Apple clang 12 were the same as clang 12.0.0
* Rust Wasm used wasmtime 0.26

| Fib | Py 3.9.4 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET6 preview3 | Open JDK 11.0.10 | C (clang 12.0.0) |
| --- | -------- | --------- | ----------- | --------- | -------------- | ---------------- | ---------------- |
| 40  | 20.3s    | 0.5s      | 0.3s        | 0.4s      | 0.5s           | 0.3s             | 0.3s             |
| 41  | 32.8s    | 0.8s      | 0.5s        | 0.7s      | 0.7s           | 0.4s             | 0.5s             |
| 42  | 53.2     | 1.4s      | 0.8s        | 1.1s      | 1.1s           | 0.7s             | 0.8s             |
| 43  | 87.7s    | 2.2s      | 1.3s        | 1.8s      | 1.9s           | 1.2s             | 1.3s             |

## Result on a Ubuntu 20.04 VM

### 2021 May

* VM on i7-10710U VM

| Fib | Py 3.9.4 | Pypy3.7-7.3.4 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET5.0.103    | Open JDK 11.0.10 | C (clang 11.0.0) | C (gcc 9.3.0) |
| --- | -------- | ------------- | --------- | ----------- | --------- | -------------- | ---------------- | ---------------- | ------------- |
| 40  | 27.3s    | 1.9s          | 0.5s      | 0.3s        | 0.4s      | 0.5s           | 0.4s             | 0.2s             | 0.3s          |
| 41  | 43.9s    | 3.0s          | 0.8s      | 0.5s        | 0.7s      | 0.8s           | 0.6s             | 0.4s             | 0.5s          |
| 42  | 71.6     | 4.6s          | 1.4s      | 0.8s        | 1.1s      | 1.1s           | 1.0s             | 0.6s             | 0.8s          |
| 43  | 115.2s   | 7.6s          | 2.2s      | 1.3s        | 1.8s      | 2.2s           | 1.6s             | 0.9s             | 1.2s          |

