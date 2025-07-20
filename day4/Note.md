## Control Flow

### If Statement

```rust
pub fn if_example() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
```bash
number is divisible by 3
```


### Match Statement

```rust
pub fn match_example() {
    let number = 5;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
}
```

```bash
five
```

### For Loop

```rust
pub fn for_loop_example() {
    for i in 1..=5 {
        println!("i = {}", i);
    }
}
```
```bash
i = 1
i = 2
i = 3
i = 4
i = 5
```

### While Loop

```rust
pub fn while_loop_example() {
    let mut i = 1;

    while i <= 5 {
        println!("i = {}", i);
        i += 1;
    }
}
```
```bash
i = 1
i = 2
i = 3
i = 4
i = 5
```