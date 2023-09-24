// Rust don't care about the order of function definitions
fn fn_order_does_not_matter() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
    expression();

    let x = five();
    println!("The value of x is: {x}");

    fn_order_does_not_matter();

    let x = plus_one(5);
    println!("The value of x is: {x}");

    let x = return_early();
    println!("The value of return early is: {x}");
}

// define type for params
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    // let y = ...; is a statement
    // = { ... } is an expression
    let y = {
        let x = 3; // statement
        x + 1 // x + 1 itself is an expression
    };
    println!("The value of y is: {y}");
}

/**
 *  In TypeScript:
 *
 *  function five(): number {
 *      return 5
 *  }
 */
// -> after function () is a return type declaration
// can return early with `return`, but the last expression is the return value by default
fn five() -> i32 {
    5 // last expression, so returns 5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // this is a statement, so returns () (unit type) which is not i32 so it won't compile
}

fn return_early() -> i32 {
    return 1; // return statement, value of 1
    2 // expression, but won't be executed
}
