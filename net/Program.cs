using System;
using System.Diagnostics;

namespace foo
{
    class Program
    {
        static uint fib(uint i)
        {
            if (i <= 1) return i;
            return fib(i - 1) + fib(i - 2);
        }

        static void Main(string[] args)
        {
            Stopwatch sw = new Stopwatch();

            for (uint i = 40; i < 44; i++)
            {
                sw.Reset();
                sw.Start();
                uint r = fib(i);
                sw.Stop();
                long t = sw.ElapsedMilliseconds;
                Console.WriteLine($"{t}, fib({i}) = {r}");
            }
        }
    }
}
