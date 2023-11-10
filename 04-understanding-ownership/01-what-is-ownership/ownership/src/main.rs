fn main() {
    // Scope
    let s = "hello";

    // block scope?
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid


    // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    // The double colon :: operator allows us to namespace this particular from function under the String type 
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
