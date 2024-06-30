TARGET = ./dist/index.js

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build:
	npx tsc

run: default
	node ./dist/index.js

clean:
	rm -rf dist/*
