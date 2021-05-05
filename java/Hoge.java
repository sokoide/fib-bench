public class Hoge {
    private static long fib(long i) {
        if(i<=1) return i;
        return fib(i-1)+fib(i-2);
    }
    public static void main(String[] args){
        for(long i=40;i<44;i++){
            long start = System.nanoTime();
            long result = fib(i);
            long end = System.nanoTime();
            System.out.printf("%d, fib(%d)=%d\n", end-start, i, result);
        }
    }
}