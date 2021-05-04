package main

import (
	"fmt"
	"time"
)

func fib(a uint) uint {
	if a <= 1 {
		return a
	}
	return fib(a-1) + fib(a-2)
}

func main() {
	var i uint
	for i = 40; i < 44; i++ {
		start := time.Now()
		result := fib(i)
		end := time.Now()
		fmt.Printf("%d, fib(%d}=%d\n", end.Sub(start), i, result)
	}
}
