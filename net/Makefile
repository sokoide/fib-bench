TARGET = net-fib

.PHONY: default build run clean

$(TARGET): build

build:
	dotnet build --framework net8.0 --configuration Release fib.csproj

build6:
	dotnet build --framework net6.0  --configuration Release fib6.csproj

run:
	dotnet run --framework net8.0 --configuration Release --project fib.csproj

run6:
	dotnet run --framework net6.0 --configuration Release --project fib6.csproj

clean:
	rm -rf bin obj
