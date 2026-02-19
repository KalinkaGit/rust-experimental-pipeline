use rust_pipeline_demo::{add, divide, fibonacci, is_prime, multiply, subtract};

fn main() {
    println!("=== Rust Pipeline Demo ===\n");

    println!("Arithmétique :");
    println!("  add(10, 5)      = {}", add(10, 5));
    println!("  subtract(10, 5) = {}", subtract(10, 5));
    println!("  multiply(10, 5) = {}", multiply(10, 5));
    println!("  divide(10, 5)   = {:?}", divide(10, 5));
    println!("  divide(10, 0)   = {:?}", divide(10, 0));

    println!("\nFibonacci (0..=10) :");
    let fib_seq: Vec<u64> = (0..=10).map(fibonacci).collect();
    println!("  {:?}", fib_seq);

    println!("\nNombres premiers jusqu'à 30 :");
    let primes: Vec<u64> = (2..=30).filter(|&n| is_prime(n)).collect();
    println!("  {:?}", primes);
}
