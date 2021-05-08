TARGET = java-fib

.PHONY: build run clean

build:
	javac -Xlint:all Hoge.java

run:
	java -server -XX:+TieredCompilation Hoge

clean:
	rm -rf *.class
