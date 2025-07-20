fn main() {
    // --- 1. Ownership and Move ---
    // s1 owns a String allocated on the heap.
    let s1 = String::from("I am a string");

    // Ownership of the String data is moved from s1 to s2.
    // s1 is no longer valid.
    let s2 = s1;

    println!("s2 holds the value: '{}'", s2);
    // The next line is commented out because it would cause a compile error:
    // println!("s1 = {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("--- Move example complete ---");


    // --- 2. The `clone()` method for a deep copy ---
    let original = String::from("clone me");
    // .clone() creates a new, independent copy of the data on the heap.
    let cloned = original.clone();

    println!("Both are valid: original = '{}', cloned = '{}'", original, cloned);
    println!("--- Clone example complete ---");


    // --- 3. The `Copy` trait for stack-only data ---
    // i32 is a fixed-size type stored entirely on the stack.
    let x = 42;
    // The value of x is copied to y. No ownership is moved.
    let y = x;

    println!("Both x and y are valid: x = {}, y = {}", x, y);
    println!("--- Copy example complete ---");


    // --- 4. Ownership in Functions ---
    let my_string = String::from("send me to a function");

    // my_string is MOVED into the function `takes_ownership`.
    // It can no longer be used in main() after this call.
    takes_ownership(my_string);

    let my_number = 100;

    // my_number is COPIED into the function `makes_copy`.
    // It can still be used in main() because i32 has the Copy trait.
    makes_copy(my_number);
    println!("I can still use my_number after the function call: {}", my_number);
    println!("--- Function ownership example complete ---");
}

// This function takes ownership of the String passed to it.
fn takes_ownership(some_string: String) {
    println!("I now own this string: '{}'", some_string);
} // `some_string` goes out of scope here, and `drop` is called. The memory is freed.

// This function takes a copy of the integer.
fn makes_copy(some_integer: i32) {
    println!("I have a copy of this number: '{}'", some_integer);
} // `some_integer` goes out of scope, but nothing special happens.