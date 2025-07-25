fn main() {
    println!("Hello, world!");

    // Vec example
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);

    // HashMap example
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("HashMap: {:?}", scores);

    // Iteration example
    for i in &v {
        println!("Vector element: {}", i);
    }

    for (key, value) in &scores {
        println!("HashMap element: {}: {}", key, value);
    }
}
