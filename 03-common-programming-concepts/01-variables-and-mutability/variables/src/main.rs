fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {}", x);

    // Wait WHAT?? there was const!! so let is immutable, const is constant, never immutable, and let mut is mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // x = 5
    let x = 5;
    // x = 5 + 1 so 6
    let x = x + 1;

    {
        // x = 6 * 2 so 12 only in inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // x = 6 out of scope
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Needs shadowing, not mutating cuz it's a different type
    let mut spaces = "   ";
    spaces = spaces.len();
}
