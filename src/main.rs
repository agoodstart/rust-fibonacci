fn main() {
    println!("{:?}", check(0))
}

fn check(n: i32) -> Result<i32, &'static str> {
    if n <= 0 {
        Err("Incorrect input")
    } else {
        Ok(fibonacci(n))
    }
}

fn fibonacci(n: i32) -> i32 {
    if n==1 {
        0
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

