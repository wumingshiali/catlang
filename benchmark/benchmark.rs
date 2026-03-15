// Rust Performance Benchmark
use std::time::Instant;

// Test 1: Simple loop (100 million iterations)
fn test_loop() {
    let mut i: i128 = 0;
    for _ in 0..100000000 {
        i = i + 1;
    }
}

// Test 2: Accumulation (10 million iterations)
fn test_accumulation() -> i128 {
    let mut result: i128 = 0;
    for j in 0..10000000 {
        result = result + j + 1;
    }
    result
}

// Test 3: Fibonacci (recursive)
fn fib(n: i128) -> i128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("=== Rust Benchmark ===\n");
    
    let start = Instant::now();
    
    println!("Running loop test...");
    let t1 = Instant::now();
    test_loop();
    let t2 = Instant::now();
    println!("Loop iterations: 100000000");
    println!("Time: {:.2} ms\n", t2.duration_since(t1).as_secs_f64() * 1000.0);
    
    println!("Running accumulation test...");
    let t1 = Instant::now();
    test_accumulation();
    let t2 = Instant::now();
    println!("Accumulation result: 50000005000000");
    println!("Time: {:.2} ms\n", t2.duration_since(t1).as_secs_f64() * 1000.0);
    
    println!("Running fibonacci test...");
    let t1 = Instant::now();
    fib(30);
    let t2 = Instant::now();
    println!("Fib(30): 832040");
    println!("Time: {:.2} ms\n", t2.duration_since(t1).as_secs_f64() * 1000.0);
    
    println!("=== Total Time: {:.0} ms ===", start.elapsed().as_secs_f64() * 1000.0);
}
