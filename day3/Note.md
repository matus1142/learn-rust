## Function
#### Defining a Function

You define a function in Rust using the fn keyword. The convention is to use snake_case for function names (all lowercase with underscores between words).

```bash
   fn main() {
       println!("Hello from main!");
       another_function();
   }
   
   fn another_function() {
       println!("Hello from another_function!");
   }
```

#### Parameters

You can pass values to functions through parameters. You must declare the type of each parameter.
```bash
   fn main() {
       print_sum(10, 20);
   }
   
   fn print_sum(x: i32, y: i32) {
       println!("The sum is: {}", x + y);
   }
```

#### Return Values

Functions can return a value. You specify the return type after an arrow ->. The last expression in the function is automatically returned.
```bash
 fn main() {
     let sum = add(10, 20);
     println!("The sum is: {}", sum);
 }
 
 fn add(x: i32, y: i32) -> i32 {
     x + y // No semicolon means this is an expression and will be returned
 }
```

You can also use the return keyword to return early from a function.
```bash
 fn main() {
     let result = check_number(5);
     println!("The result is: {}", result);
 }
 
 fn check_number(x: i32) -> i32 {
     if x > 10 {
         return x - 10;
     }
       x
 }
```
Statements vs. Expressions

  - Statements are instructions that perform an action but don't return a value (e.g., let x = 5;).
  - Expressions evaluate to a value (e.g., 5 + 6).

In Rust, if you add a semicolon to the end of an expression, it becomes a statement and won't return a value. This is a common source of errors for
beginners.