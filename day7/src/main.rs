fn main() {

    //- String

    // Create a new, empty String
    let mut s1 = String::new();
    s1.push_str("Hello, "); // Append a string slice

    // Create a String from a string literal (&str)
    let s2 = String::from("world");

    // Another way to create a String from a &str
    let mut s3 = "initial content".to_string();
    s3.push('!'); // Append a single character

    // Concatenate strings (this consumes s2)
    let s_combined = s1 + &s2;

    println!("{}", s_combined); // Output: Hello, world
    println!("{}", s3);        // Output: initial content!


    //- &str

    // A string literal is a &str
    let s_literal: &'static str = "I am a string literal.";

    // A function that takes a string slice as an argument
    // This is the idiomatic way to accept string data in functions
    fn print_message(message: &str) {
        println!("The message is: {}", message);
    }

    // Create an owned String
    let my_string = String::from("This is an owned String.");

    // We can pass a reference to the String to the function.
    // Rust's "deref coercion" automatically converts &String into &str.
    print_message(&my_string);

    // We can also pass a string literal directly.
    print_message(s_literal);


    //- Slicing
    let s = String::from("hello world");

    // Slicing a String to get a &str
    // The indices are [inclusive..exclusive]
    let hello: &str = &s[0..5];   // from byte 0 up to (but not including) byte 5
    let world: &str = &s[6..11];  // from byte 6 up to (but not including) byte 11

    println!("First word: {}", hello); // Output: First word: hello
    println!("Second word: {}", world); // Output: Second word: world

    // Slicing a non-ASCII string
    let unicode_string = String::from("नमस्ते"); // Each character is 3 bytes in UTF-8

    // This is valid because we are slicing on character boundaries
    // 'न' is bytes 0-2, 'म' is 3-5, 'स्' is 6-8, etc.
    let namaste_part: &str = &unicode_string[0..6];
    println!("Slice of नमस्ते: {}", namaste_part); // Output: Slice of नमस्ते: नम

    // This would cause a PANIC because 1 is not a character boundary!
    // let invalid_slice = &unicode_string[0..1];

}
