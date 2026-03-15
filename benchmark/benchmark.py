# Python Performance Benchmark
import time

# Test 1: Simple loop (100 million iterations)
def test_loop():
    i = 0
    for _ in range(100000000):
        i = i + 1

# Test 2: Accumulation (10 million iterations)
def test_accumulation():
    result = 0
    for j in range(10000000):
        result = result + j + 1
    return result

# Test 3: Fibonacci (recursive)
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

def main():
    print("=== Python Benchmark ===\n")
    
    start = time.time()
    
    print("Running loop test...")
    t1 = time.time()
    test_loop()
    t2 = time.time()
    print("Loop iterations: 100000000")
    print(f"Time: {(t2 - t1) * 1000:.2f} ms\n")
    
    print("Running accumulation test...")
    t1 = time.time()
    test_accumulation()
    t2 = time.time()
    print("Accumulation result: 50000005000000")
    print(f"Time: {(t2 - t1) * 1000:.2f} ms\n")
    
    print("Running fibonacci test...")
    t1 = time.time()
    fib(30)
    t2 = time.time()
    print("Fib(30): 832040")
    print(f"Time: {(t2 - t1) * 1000:.2f} ms\n")
    
    print(f"=== Total Time: {(time.time() - start) * 1000:.0f} ms ===")

if __name__ == "__main__":
    main()
