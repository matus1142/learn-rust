
# Rust Ownership, Move, and Clone: A Deep Dive

Rust's ownership system is a core feature that ensures memory safety without needing a garbage collector (GC). It's a set of rules checked at compile time that don't add any runtime overhead.

### The Three Rules of Ownership

1.  **One Owner:** Each value in Rust has a variable that’s called its *owner*.
2.  **One at a Time:** There can only be one owner at a time.
3.  **Scope Drops:** When the owner goes out of scope, the value will be dropped, and its memory will be freed.

---

### 1. Move (The Default Behavior)

When you assign a variable that points to data on the **heap** (like a `String`, `Vec<T>`, or a `Box<T>`) to another variable, Rust *moves* ownership. It does not create a "deep copy" of the data. Instead, it copies the pointer/length/capacity information from the old variable to the new one and invalidates the original variable.

This prevents a "double free" error, where two variables might try to free the same memory when they go out of scope.

**Example:**

```rust
// --- 1. Ownership and Move ---
// s1 owns a String allocated on the heap.
let s1 = String::from("I am a string");

// Ownership of the String data is moved from s1 to s2.
// s1 is no longer valid.
let s2 = s1;

println!("s2 holds the value: '{}'", s2);
// The next line is commented out because it would cause a compile error:
// println!("s1 = {}", s1); // error[E0382]: borrow of moved value: `s1`
```
**Error Log:**
```bash
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:10:42
   |
4  |     let s1 = String::from("I am a string");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
...
8  |     let s2 = s1;
   |              -- value moved here
9  |
10 |     println!("s2 holds the value: '{}'", s1);
   |                                          ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
8  |     let s2 = s1.clone();
   |                ++++++++

For more information about this error, try `rustc --explain E0382`.
warning: `day5` (bin "day5") generated 1 warning
```

---

### 2. Clone (Making an Explicit Deep Copy)

If you need a true deep copy of heap data, you must explicitly call the `clone()` method. This creates a brand new, independent copy of the data on the heap. This can be a more expensive operation, as it involves allocating new memory and copying all the data.

**Example:**

```rust
// --- 2. The `clone()` method for a deep copy ---
let original = String::from("clone me");
// .clone() creates a new, independent copy of the data on the heap.
let cloned = original.clone();

println!("Both are valid: original = '{}', cloned = '{}'", original, cloned);
```

---

### 3. Copy (For Stack-Only Data)

Some types are stored entirely on the **stack** because they have a known, fixed size at compile time. Examples include:
- All integer types (`i32`, `u64`, etc.)
- The boolean type (`bool`)
- All floating-point types (`f64`, `f32`)
- The character type (`char`)
- Tuples, if they only contain types that are also `Copy`.

These types implement the `Copy` trait. When you assign them to another variable, Rust makes a simple bit-for-bit copy. There's no concept of "moving" because there's no heap data or ownership to transfer. Both the original and the new variable are valid.

**Example:**

```rust
// --- 3. The `Copy` trait for stack-only data ---
// i32 is a fixed-size type stored entirely on the stack.
let x = 42;
// The value of x is copied to y. No ownership is moved.
let y = x;

println!("Both x and y are valid: x = {}, y = {}", x, y);
```

---

### Ownership in Functions

Passing a variable to a function works the same way as assignment:
- Passing a heap-allocated variable (like a `String`) **moves** ownership to the function.
- Passing a stack-allocated variable (like an `i32`) **copies** the value to the function.

```rust
fn main() {
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
}

// This function takes ownership of the String passed to it.
fn takes_ownership(some_string: String) {
    println!("I now own this string: '{}'", some_string);
} // `some_string` goes out of scope here, and `drop` is called. The memory is freed.

// This function takes a copy of the integer.
fn makes_copy(some_integer: i32) {
    println!("I have a copy of this number: '{}'", some_integer);
} // `some_integer` goes out of scope, but nothing special happens.
```
