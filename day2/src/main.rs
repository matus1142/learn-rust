fn main() {
    let mut x = 5; // Mutable
    let y = 10; // Immutable
    println!("x = {}, y = {}", x, y);
    x = 1;
    y = 2;
    println!("x = {}, y = {}", x, y);
}
