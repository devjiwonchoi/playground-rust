use std::io;
use std::process;

fn main() {
    println!("Enter a positive number `n` to get the nth Fibonacci number");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Something went wrong. Please try again.");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Error: {err}. Please enter a natural number.");
            process::exit(1)
        }
    };

    let result = get_fibonacci(n);

    println!("{result}")
}

fn get_fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        get_fibonacci(n - 1) + get_fibonacci(n - 2)
    }
}
