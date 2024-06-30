function fib(a: number): number {
	if (a <= 1 ) return a;
	else return fib(a-1) + fib(a-2);
}



for(let i:number=40;i<=43;i++) {
	var st:number = performance.now()
	var n:number = fib(i)
	var ed:number = performance.now()

	console.log("fib(%d)=%d, %d ms", i, n, ed-st);
}

