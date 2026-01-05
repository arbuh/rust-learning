fn fibonacci_of(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibonacci_of(n - 1) + fibonacci_of(n - 2)
}

fn main() {
    let result = fibonacci_of(15);
    println!("Result is {result}");
}
