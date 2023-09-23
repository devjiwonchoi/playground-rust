use std::cmp::Ordering;
use std::io;
// Rng is randome number genrator trait
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // let is like const in ES6, immutable
        // let mut is like let in ES6, mutable

        // :: is an associated function of a type
        // so new() function is a String type
        let mut guess = String::new();

        io::stdin()
            // & is a reference, immutable by default, no need to copy data into memory multiple times
            .read_line(&mut guess)
            // break lines when calling multiple methods for readability
            // .expect() is like .catch() I guess..
            .expect("Failed to read line");

        // shadowing - allowing to declare an existing var name???
        // so guess on the right-hand is original..
        // trim() removes \n (new line)
        // parse() parses a string to another type, here u32
        // i32 is 32-bit integer, which is default
        // u32 is unsigned 32-bit integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // parse returns a `Result` type, which is an enum that has variants `Ok` or `Err`
            Ok(num) => num,
            // _ is a catch-all value DUH?
            Err(_) => continue,
        };

        // {} is a like `${}` in ES6 I guess..
        println!("You guessed: {guess}");
        // within an expression, an empty placeholder {} prints the result of evaluated expression
        // println!("You guessed: {}", guess);

        // match is like switch/case in JS?
        // "comparison with secret_number means Rust will infer that secret_number should be a u32 as well."
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// cargo doc --open is crazy!! it opens a browser with all the documentation of the dependencies
