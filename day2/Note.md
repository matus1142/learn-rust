## Mutable & Immutable

```bash
let mut x = 5; // Mutable
let y = 10; // Immutable
```

### Error log
```bash
error[E0384]: cannot assign twice to immutable variable `y`
 --> src/main.rs:8:5
  |
5 |     let y = 10; // Immutable
  |         - first assignment to `y`
...
8 |     y = 2;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
5 |     let mut y = 10; // Immutable
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `day2` (bin "day2") due to 1 previous error
```

## Data Types

```bash
// Scalar Types

// Integers
let a: i32 = -10;
let b: u32 = 20;

// Floating-Point Numbers
let x = 2.0; // f64
let y: f32 = 3.0; // f32

// Booleans
let t = true;
let f: bool = false;

// Characters
let c = 'z';
let z: char = 'Z';
let heart_eyed_cat = '😻';

// Compound Types

// Tuples
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // Destructuring

// Arrays
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];

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


// Enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));

// Other Common Types

// Strings
let s1: &str = "hello world"; // string slice
let s2: String = String::from("hello world"); // String


// Slices
let a: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3]; // A slice of the array a
```