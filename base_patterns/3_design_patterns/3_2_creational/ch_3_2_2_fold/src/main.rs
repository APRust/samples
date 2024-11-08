fn fibonacci(n: u32) -> u32 {
    (0..n + 1).fold(0, |a, b| a + b)
}

fn factorial(n: u32) -> u32 {
    match n {
        0..=1 => 1,
        _ => (1..n + 1).fold(1, |a, b| a * b),
    }
}

fn main() {
    let n = 5;
    println!("fibonacci with index {} = {}", n, fibonacci(n));
    println!("factorial with index {} = {}", n, factorial(n));
}
