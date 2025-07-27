// An enum where variants hold different types and amounts of data
enum Message {
    Quit, // No data
    Write(String), // Holds a String
    Move { x: i32, y: i32 }, // Holds an anonymous struct
    ChangeColor(i32, i32, i32), // Holds three i32 values
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message: has no data.");
        }
        Message::Write(text) => {
            // The `text` variable is bound to the String value
            println!("Write message: {}", text);
        }
        Message::Move { x, y } => {
            // We can destructure the x and y values
            println!("Move message: move to x={}, y={}", x, y);
        }
        Message::ChangeColor(r, g, b) => {
            // We can bind the values to r, g, and b
            println!("ChangeColor message: R={}, G={}, B={}", r, g, b);
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Write(String::from("Hello, Rust!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 128),
    ];

    for msg in messages {
        process_message(msg);
    }
}