fn main() {
    println!("Tempture 30C is {}F", celsius2fahrenheit(31.0));
    println!("10th Fibonacci number is {}", fibonacci(10));
}

fn celsius2fahrenheit(templture: f64) -> f64 {
    templture * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
