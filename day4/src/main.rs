mod if_example;
mod match_example;
mod for_loop_example;
mod while_loop_example;

fn main() {
    println!("--- If Example ---");
    if_example::if_example();

    println!("\n--- Match Example ---");
    match_example::match_example();

    println!("\n--- For Loop Example ---");
    for_loop_example::for_loop_example();

    println!("\n--- While Example ---");
    while_loop_example::while_loop_example();
}