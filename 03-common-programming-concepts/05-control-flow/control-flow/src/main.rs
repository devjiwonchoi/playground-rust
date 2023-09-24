fn main() {
    let number = 3;

    // if else looks the same with JS except no () around the condition
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Syntax like JS, but no loose equality like PY
    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // conditional assignment `? :` in JS
    let number = if condition { 5 } else { 6 };

    // This also works, but I guess not idiomatic
    // let number = if condition {
    //     5
    // } else {
    //     6
    // };

    // This doesn't work, because the types are different
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
