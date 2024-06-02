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
rustup target add wasm32-unknown-unknown
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
# change net5.0 to net6.0 in fib.csproj to compile with .NET6 SDK
dotnet run -c release
```

* Java

```bash
javac Hoge.java
java Hoge
```

* C

```
clang-11 -O3 -lc main.c -o main
./main
```

## Result on M1 Mac mini

* Times are all in seconds
* clang 11.1.0 and Apple clang 12 were the same as clang 12.0.0

| Fib | Py 3.9.4 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET6 preview3 | Open JDK 11.0.10 | C (clang 12.0.0) |
| --- | -------- | --------- | ----------- | --------- | -------------- | ---------------- | ---------------- |
| 40  | 20.3s    | 0.5s      | 0.3s        | 0.4s      | 0.5s           | 0.3s             | 0.3s             |
| 41  | 32.8s    | 0.8s      | 0.5s        | 0.7s      | 0.7s           | 0.4s             | 0.5s             |
| 42  | 53.2     | 1.4s      | 0.8s        | 1.1s      | 1.1s           | 0.7s             | 0.8s             |
| 43  | 87.7s    | 2.2s      | 1.3s        | 1.8s      | 1.9s           | 1.2s             | 1.3s             |

## Result on a Ubuntu 20.04 VM

* VM on i7-10710U VM

| Fib | Py 3.9.4 | Pypy3.7-7.3.4 | Pyjion 0.15.0 | Go 1.16.3 | Rust 1.51.0 | Rust Wasm | .NET5.0.103    | Open JDK 11.0.10 | C (clang 11.0.0) | C (gcc 9.3.0) |
| --- | -------- | ------------- | ------------- | --------- | ----------- | --------- | -------------- | ---------------- | ---------------- | ------------- |
| 40  | 27.3s    | 1.9s          | 36.7s         | 0.5s      | 0.3s        | 0.4s      | 0.5s           | 0.4s             | 0.2s             | 0.3s          |
| 41  | 43.9s    | 3.0s          | 59.9s         | 0.8s      | 0.5s        | 0.7s      | 0.8s           | 0.6s             | 0.4s             | 0.5s          |
| 42  | 71.6     | 4.6s          | 97.2s         | 1.4s      | 0.8s        | 1.1s      | 1.1s           | 1.0s             | 0.6s             | 0.8s          |
| 43  | 115.2s   | 7.6s          | 156.6s        | 2.2s      | 1.3s        | 1.8s      | 2.2s           | 1.6s             | 0.9s             | 1.2s          |

