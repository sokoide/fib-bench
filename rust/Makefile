TARGET = rs-fib

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build:
	cargo build --release

run: $(TARGET)
	cargo run --release

clean:
	cargo clean
