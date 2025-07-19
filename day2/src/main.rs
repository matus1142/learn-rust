// Mutable & Immutable
fn main() {
    let mut x = 5; // Mutable
    let y = 10; // Immutable
    println!("x = {}, y = {}", x, y);
    x = 1;
    y = 2;
    println!("x = {}, y = {}", x, y);
}

// Data types
fn main() {
    // Scalar Types

    // Integers
    let a: i32 = -10;
    let b: u32 = 20;
    println!("Integers: a = {}, b = {}", a, b);

    // Floating-Point Numbers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Floats: x = {}, y = {}", x, y);

    // Booleans
    let t = true;
    let f: bool = false;
    println!("Booleans: t = {}, f = {}", t, f);

    // Characters
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("Characters: c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    // Compound Types

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("Tuple destructured: y = {}", y);
    println!("Tuple access by index: tup.0 = {}", tup.0);


    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("Array elements: first = {}, second = {}", first, second);

    // Custom Data Types

    // Structs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Struct field: user1.email = {}", user1.email);


    // Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Write(text) => println!("Enum match: Message is {}", text),
        _ => (),
    }

    // Other Common Types

    // Strings
    let s1: &str = "hello world"; // string slice
    let s2: String = String::from("hello world"); // String
    println!("String slice: {}", s1);
    println!("String: {}", s2);


    // Slices
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3]; // A slice of the array a
    println!("Slice: {:?}", slice);
}