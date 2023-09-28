Define **C-like** debug trace macro: `enter!()` and `leave!()`.

```rust
use dmacro::*;

fn main() {
    enter!();
    println!("Hello, world!");
    leave!();
}
```
