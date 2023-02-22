use test_criterion::{fibonacci, fibonacci_rec, Fibonacci};

fn main() {
    println!("{}", Fibonacci::default().nth(20).unwrap());
    println!("{}", fibonacci(20));
    println!("{}", fibonacci_rec(20));
}
