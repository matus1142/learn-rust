# Rust Concepts: Ownership, References, and Borrowing

This document explains three core and related concepts in Rust that are crucial for writing safe and efficient code: Ownership, References (`&`), and Mutable References (`&mut`). These concepts allow Rust to guarantee memory safety at compile time without needing a garbage collector.

---

## 1. Ownership

Ownership is Rust's most unique feature. It's a set of rules that the compiler checks at compile time.

### The Three Ownership Rules:

1.  **Each value in Rust has a variable that’s called its *owner*.**
2.  **There can only be one owner at a time.**
3.  **When the owner goes out of scope, the value will be dropped.**

This system prevents "dangling pointers" (pointing to memory that has been freed) and "data races" (two or more pointers accessing the same data at the same time, where at least one is writing).

### Example: Move

When you pass a complex data type like a `String` (which is stored on the heap) to another function, its ownership is *moved*.

```rust
fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1); // s1's ownership is moved into the function...
                         // ...and so is no longer valid here.

    // println!("{}", s1); // This line would cause a COMPILE ERROR! s1 was moved.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
```

---

## 2. References & Borrowing (`&`)

What if we want to let a function use a value but not take ownership of it? This is where *borrowing* comes in. We can create a *reference* to the value. A reference is like a pointer in that it’s an address we can follow to access the data stored at that address.

- We use the `&` ampersand symbol to create a reference.
- References are **immutable** by default. You cannot change the value you are borrowing.

### The Rules of References

1.  At any given time, you can have *either* one mutable reference *or* any number of immutable references.
2.  References must always be valid.

### Example: Immutable Borrow

```rust
fn main() {
    let s1 = String::from("hello");

    // We pass a reference to s1 to the function.
    // s1 is "borrowed", not "moved".
    let len = calculate_length(&s1);

    // Because s1 was only borrowed, it is still valid here.
    println!("The length of '{}' is {}.", s1, len);
}

// The function signature uses &String to show it takes a reference.
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String.
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens. The original value is not dropped.
```

---

## 3. Mutable References (`&mut`)

If you want to change a borrowed value, you need a *mutable reference*.

- You use `&mut` to create a mutable reference.
- The original variable must also be declared as `mut`.
- **The Big Rule:** You can only have **one** mutable reference to a particular piece of data in a particular scope. This is how Rust prevents data races at compile time.

### Example: Mutable Borrow

```rust
fn main() {
    // The variable must be mutable to allow a mutable borrow.
    let mut s = String::from("hello");

    println!("Original string: {}", s);

    // Pass a mutable reference to the function.
    change(&mut s);

    // The original string has been changed.
    println!("Modified string: {}", s);
}

// The function takes a mutable reference to a String.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
This rule is very strict. While you have a mutable reference, you cannot create any other references (mutable or immutable) to that value until the mutable reference goes out of scope.

---
## Final Runnable Example

Here is a complete example you can run that demonstrates all these concepts together.

```rust
// This function takes an immutable reference (a "borrow").
// It can read the data but not change it.
fn inspect_string(s: &String) {
    println!("Inspecting string: '{}'", s);
}

// This function takes a mutable reference.
// It can read and modify the data.
fn modify_string(s: &mut String) {
    s.push_str(" and goodbye");
    println!("String modified inside function.");
}

fn main() {
    // 1. OWNERSHIP
    // `s1` is the owner of the String data "hello world".
    // It must be `mut` because we intend to borrow it mutably later.
    let mut s1 = String::from("hello world");
    println!("Original s1: '{}'", s1);

    // 2. IMMUTABLE BORROW (&)
    // We can create multiple immutable references.
    let r1 = &s1;
    let r2 = &s1;
    println!("Immutable borrows r1: '{}', r2: '{}'", r1, r2);

    // We can use our inspect function with an immutable borrow.
    inspect_string(&s1);

    // `r1` and `r2` go out of scope here.

    // 3. MUTABLE BORROW (&mut)
    // Now we create a mutable reference.
    // From this point until `r3` is last used, we cannot have ANY other references to `s1`.
    let r3 = &mut s1;
    // modify_string(r3); // This is one way to use it
    r3.push_str("!"); // Or we can use it directly

    println!("s1 after direct mutable borrow modification: '{}'", r3);

    // We can also pass the mutable borrow to another function.
    // Note: We pass `r3` itself, which is `&mut String`.
    modify_string(r3);

    // `r3`'s scope ends here, so the mutable borrow is over.

    // Now that the mutable borrow is finished, we can use `s1` again.
    println!("Final s1 value: '{}'", s1);
}

```
```bash
Original s1: 'hello world'
Immutable borrows r1: 'hello world', r2: 'hello world'
Inspecting string: 'hello world'
s1 after direct mutable borrow modification: 'hello world!'
String modified inside function.
Final s1 value: 'hello world! and goodbye'
```
