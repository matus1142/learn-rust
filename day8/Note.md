
### Core Collection Types in Rust

In Rust, the standard library `std::collections` provides several very useful data structures known as **collections**. While there are many, the most common and important ones to start with are:

1.  **`Vec<T>`** (Vector): A growable, heap-allocated list of items of the same type `T`. Think of it like an array or list in other languages that can change in size.
2.  **`HashMap<K, V>`**: A data structure that stores key-value pairs. It uses a hashing function to determine how to store the data, making lookups for a specific key very fast.

### 1. `Vec<T>` - The Vector

A vector allows you to store a variable number of values next to each other in memory.

#### Creating a Vector

You can create a new, empty vector with `Vec::new()`:

```rust
let mut v: Vec<i32> = Vec::new();
```

Rust can often infer the type, so if you add elements right away, you don't need the type annotation:

```rust
let mut v = Vec::new();
v.push(5); // Now Rust knows v is Vec<i32>
```

For convenience, Rust provides the `vec![]` macro, which is the most common way to create a vector:

```rust
let v = vec![1, 2, 3]; // Creates a Vec<i32> with initial values
```

#### Adding and Accessing Elements

*   **`push()`**: Adds an element to the end of the vector.
*   **`[]`**: Accesses an element by its index. **Warning:** This will cause your program to `panic!` (crash) if you provide an index that is out of bounds.
*   **`get()`**: A safer way to access an element. It returns an `Option<T>`, which will be `Some(&T)` if the element exists, or `None` if the index is out of bounds. This allows you to handle the "not found" case gracefully.

```rust
let mut v = vec![10, 20, 30];
v.push(40);

// Accessing elements
let third: &i32 = &v[2]; // third is 30
println!("The third element is {}", third);

// Safer access with get()
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// This would panic!
// let does_not_exist = &v[100];
```

### 2. `HashMap<K, V>` - The Hash Map

A hash map stores data in key-value pairs. It's useful when you want to look up data not by an index, but by a unique identifier (the key).

#### Creating a Hash Map

Similar to vectors, you can create a new, empty hash map with `HashMap::new()`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

#### Accessing Values

You can use the `get()` method with a key to retrieve the corresponding value. It returns an `Option<&V>`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

let team_name = String::from("Blue");
let score = scores.get(&team_name); // Returns Some(&10)

match score {
    Some(s) => println!("The score is {}", s),
    None => println!("Team not found."),
}
```

### 3. Iteration

Iterating over a collection is a fundamental operation. Rust's `for` loop is the most common way to do this.

#### Iterating Over a `Vec`

You can iterate over the elements of a vector in a few different ways, depending on what you need to do.

*   **Immutable iteration (`&v`)**: This gives you an immutable reference to each element. You can read the elements, but not change them.

    ```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    ```

*   **Mutable iteration (`&mut v`)**: This gives you a mutable reference, allowing you to change the values in the vector.

    ```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Use the dereference operator (*) to change the value
    }
    println!("{:?}", v); // Prints [150, 82, 107]
    ```

*   **Owned iteration (`v`)**: This takes ownership of the vector and its elements. After the loop, the vector cannot be used again. This is useful when you want to move the elements into a new data structure.

    ```rust
    let v = vec![100, 32, 57];
    for i in v { // v is moved here and cannot be used after the loop
        println!("{}", i);
    }
    // println!("{:?}", v); // This would cause a compile error
    ```

#### Iterating Over a `HashMap`

When you iterate over a `HashMap`, you get key-value pairs.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// The order of iteration is not guaranteed!
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Summary

*   **`Vec<T>`**: A dynamic array for storing a list of items of the same type.
*   **`HashMap<K, V>`**: For storing key-value pairs, allowing for fast lookups.
*   **Iteration**: The `for` loop is your primary tool.
    *   Use `&collection` for read-only access.
    *   Use `&mut collection` to modify elements.
    *   Use `collection` to take ownership of the elements.
