Define **C-like** debug trace macro: `enter!()` and `leave!()`.

```rust
use dmacro::{enter,leave};

fn main() {
    enter!();
    println!("Hello, world!");
    leave!();
}
```