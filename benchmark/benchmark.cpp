// C++ Performance Benchmark (MSVC)
#include <iostream>
#include <chrono>

using namespace std;
using namespace std::chrono;

// Test 1: Simple loop (100 million iterations)
void test_loop() {
    long long i = 0;
    for (long long x = 0; x < 100000000; x++) {
        i = i + 1;
    }
}

// Test 2: Accumulation (10 million iterations)
long long test_accumulation() {
    long long result = 0;
    for (long long j = 0; j < 10000000; j++) {
        result = result + j + 1;
    }
    return result;
}

// Test 3: Fibonacci (recursive)
long long fib(long long n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

int main() {
    cout << "=== C++ (MSVC) Benchmark ===" << endl << endl;
    
    auto start = high_resolution_clock::now();
    
    cout << "Running loop test..." << endl;
    auto t1 = high_resolution_clock::now();
    test_loop();
    auto t2 = high_resolution_clock::now();
    cout << "Loop iterations: 100000000" << endl;
    cout << "Time: " << duration_cast<microseconds>(t2 - t1).count() / 1000.0 << " ms" << endl << endl;
    
    cout << "Running accumulation test..." << endl;
    t1 = high_resolution_clock::now();
    test_accumulation();
    t2 = high_resolution_clock::now();
    cout << "Accumulation result: 50000005000000" << endl;
    cout << "Time: " << duration_cast<microseconds>(t2 - t1).count() / 1000.0 << " ms" << endl << endl;
    
    cout << "Running fibonacci test..." << endl;
    t1 = high_resolution_clock::now();
    fib(30);
    t2 = high_resolution_clock::now();
    cout << "Fib(30): 832040" << endl;
    cout << "Time: " << duration_cast<microseconds>(t2 - t1).count() / 1000.0 << " ms" << endl << endl;
    
    auto end = high_resolution_clock::now();
    cout << "=== Total Time: " << duration_cast<milliseconds>(end - start).count() << " ms ===" << endl;
    
    return 0;
}
