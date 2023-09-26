use std::io;
use std::process;

fn main() {
    println!("Enter a natural number `n` to get the nth Fibonacci number (Max 10)");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Something went wrong. Please try again.");

    let n: u8 = match n.trim().parse() {
        Ok(num) => {
            if num > 10 {
                eprintln!("Warning: maximum input number is 10.");
                process::exit(1)
            } else {
                num
            }
        }
        Err(err) => {
            eprintln!("Error: {err}. Please enter a natural number.");
            process::exit(1)
        }
    };

    if n > 10 {
        eprintln!("Maximum input number is 100.");
        process::exit(1)
    }

    let result = get_fibonacci(n);

    println!("{result}")
}

fn get_fibonacci(n: u8) -> u8 {
    if n < 2 {
        n
    } else {
        get_fibonacci(n - 1) + get_fibonacci(n - 2)
    }
}
