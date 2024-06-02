import time
import pyjion


def fib(i: int) -> int:
    if i <= 1:
        return i
    return fib(i - 1) + fib(i - 2)


def main():
    pyjion.enable()
    for i in range(40, 44):
        start = time.time()
        result = fib(i)
        end = time.time()
        print("%f, fib(%d)=%d" % (end - start, result, i))


if __name__ == '__main__':
    main()
