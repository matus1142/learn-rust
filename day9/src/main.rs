// 1. Defining a struct
struct Book {
    title: String,
    author: String,
    publication_year: u32,
    is_available: bool,
}

fn main() {
    // 2. Instantiating a struct
    let mut book1 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        publication_year: 2018,
        is_available: true,
    };

    // 3. Accessing fields
    println!("--- Initial Book Details ---");
    println!("Title: {}", book1.title);
    println!("Author: {}", book1.author);
    println!("Publication Year: {}", book1.publication_year);
    println!("Available: {}", book1.is_available);

    // Modifying a field (instance must be `mut`)
    println!("--- Checking out the book... ---");
    book1.is_available = false;

    // Accessing the modified field
    println!("Available: {}", book1.is_available);

    // You can also pass the struct to a function
    print_book_summary(&book1);
}

// A function that takes a reference to a Book struct
fn print_book_summary(book: &Book) {
    println!("--- Book Summary ---");
    println!("'{}' by {} ({})", book.title, book.author, book.publication_year);
}