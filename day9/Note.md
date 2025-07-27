# Understanding Structs in Rust

A `struct`, short for *structure*, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. They are a fundamental concept in Rust for creating more complex data types.

---

## 1. Creating a Struct

You create a struct using the `struct` keyword, followed by the name of the struct, and then a block of curly braces `{}` containing the names and types of its data fields. Each piece of data within the struct is called a **field**.

### Example:

Let's define a struct to store information about a `Book`.

```rust
struct Book {
    title: String,
    author: String,
    publication_year: u32,
    is_available: bool,
}
```

In this example, the `Book` struct has four fields:
- `title` and `author` are of type `String`.
- `publication_year` is an unsigned 32-bit integer.
- `is_available` is a boolean.

---

## 2. Instantiating a Struct

Once you have defined a struct, you can create an *instance* of it. To do this, you state the name of the struct and then provide concrete values for each of the fields in `key: value` pairs inside curly braces.

### Example:

Let's create an instance of the `Book` struct we defined earlier.

```rust
// First, we need the struct definition.
struct Book {
    title: String,
    author: String,
    publication_year: u32,
    is_available: bool,
}

// Now, let's create an instance of the Book struct.
let book1 = Book {
    title: String::from("The Rust Programming Language"),
    author: String::from("Steve Klabnik and Carol Nichols"),
    publication_year: 2018,
    is_available: true,
};
```
*Note: The order of fields does not need to match the order in the struct definition when you are instantiating it.*

---

## 3. Accessing and Modifying Fields

To get or change a specific value from a struct instance, you use dot notation (`.`).

To modify a field, the instance of the struct must be declared as mutable using the `mut` keyword.

### Example:

```rust
// Assuming `book1` is the instance we created in the previous step.

// Accessing the title and author
println!("The book '{}' was written by {}.", book1.title, book1.author);
// Output: The book 'The Rust Programming Language' was written by Steve Klabnik and Carol Nichols.

// To modify fields, the instance must be mutable.
// Let's create a mutable instance.
let mut book2 = Book {
    title: String::from("Rust for Rustaceans"),
    author: String::from("Jon Gjengset"),
    publication_year: 2021,
    is_available: true,
};

// Let's say someone borrows the book. We can update the `is_available` field.
book2.is_available = false;

println!("Is '{}' available? {}", book2.title, book2.is_available);
// Output: Is 'Rust for Rustaceans' available? false
```
