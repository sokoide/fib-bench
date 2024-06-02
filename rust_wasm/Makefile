TARGET = rs-fib

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build:
	cargo build --release --target wasm32-unknown-unknown

clean:
	cargo clean
