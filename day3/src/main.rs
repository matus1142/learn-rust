fn main() {
    let result = check_number(5);
    println!("The result is: {}", result);
    let result = check_number(100);
    println!("The result is: {}", result);
}

fn check_number(x: i32) -> i32 {
    if x > 10 {
        return x - 10;
    }
    x // this is an expression and will be returned  
}
