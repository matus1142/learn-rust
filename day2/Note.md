### Mutable & Immutable

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
