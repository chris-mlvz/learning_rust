use std::io;

fn main() {
    println!("Enter a number to calculate fibonacci:");
    let mut user_number = String::new();
    io::stdin()
        .read_line(&mut user_number)
        .expect("Failed to read line");
    let user_number: u64 = user_number.trim().parse().expect("Not a number");
    println!("Fibonacci {} = {}", user_number, fibonacci(user_number));
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut first = 0;
    let mut second = 1;
    for _ in 2..=n {
        let temp = first + second;
        first = second;
        second = temp;
    }
    second
}
