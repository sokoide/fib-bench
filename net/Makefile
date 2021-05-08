TARGET = net-fib

.PHONY: default build run clean

$(TARGET): build

build:
	dotnet build --configuration Release

run:
	dotnet run --configuration Release

clean:
	rm -rf bin obj
