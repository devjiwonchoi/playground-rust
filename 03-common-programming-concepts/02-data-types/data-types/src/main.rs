// Rust `panicks` - exiting with error on debug mode
fn main() {
    // Since variable guess may have several types, we need to specify it.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // Scalar Types - Single Value
    // 1. Integers - bit (default i32)
    //   - Signed (shown with + or - sign): i8, i16, i32, i64, i128, isize
    //   - Unsigned (only positive): u8, u16, u32, u64, u128, usize
    //   - Number literals available: Decimal, Hex, Octal, Binary, Byte (u8 only)
    // 2. Floating-Point Numbers - f32 f64 (default)
    //   - Numeric operation looks same with JS
    // 3. Booleans - bool
    // 4. Characters - char (single quotes)
    //   - Unicode Scalar Value (4 bytes each), differ from string liternals (double quotes)

    // Compound Types - Multiple Values in one type
    // 1. Tuples - Fixed length, can't grow or shrink
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("{x} {y} {z}");
    // ChatGPT:
    // "In Rust, you cannot directly access tuple elements using dot notation inside a string interpolation.
    // You should use curly braces {} to interpolate the values correctly."
    println!("{} {} {}", tup.0, tup.1, tup.2); // Accessing by index

    // tuple without any value - unit

    // 2. Arrays - EVERY ELEMENT MUST HAVE SAME TYPE. Wow.. Also, fixed length
    // prefer `vector` more than `array`
    // I guess it's like a constant array
    let a = [1, 2, 3, 4, 5];
    // So accessing array looks same with JS
    // Tuple uses `.` but array uses `[]`
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    // specify type by let variable = [type; length] = [values];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    // initial value for array
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
}
