// This function takes an immutable reference (a "borrow").
// It can read the data but not change it.
fn inspect_string(s: &String) {
    println!("Inspecting string: '{}'", s);
}

// This function takes a mutable reference.
// It can read and modify the data.
fn modify_string(s: &mut String) {
    s.push_str(" and goodbye");
    println!("String modified inside function.");
}

fn main() {
    // 1. OWNERSHIP
    // `s1` is the owner of the String data "hello world".
    // It must be `mut` because we intend to borrow it mutably later.
    let mut s1 = String::from("hello world");
    println!("Original s1: '{}'", s1);

    // 2. IMMUTABLE BORROW (&)
    // We can create multiple immutable references.
    let r1 = &s1;
    let r2 = &s1;
    println!("Immutable borrows r1: '{}', r2: '{}'", r1, r2);

    // We can use our inspect function with an immutable borrow.
    inspect_string(&s1);

    // `r1` and `r2` go out of scope here.

    // 3. MUTABLE BORROW (&mut)
    // Now we create a mutable reference.
    // From this point until `r3` is last used, we cannot have ANY other references to `s1`.
    let r3 = &mut s1;
    // modify_string(r3); // This is one way to use it
    r3.push_str("!"); // Or we can use it directly

    println!("s1 after direct mutable borrow modification: '{}'", r3);

    // We can also pass the mutable borrow to another function.
    // Note: We pass `r3` itself, which is `&mut String`.
    modify_string(r3);

    // `r3`'s scope ends here, so the mutable borrow is over.

    // Now that the mutable borrow is finished, we can use `s1` again.
    println!("Final s1 value: '{}'", s1);
}