# Define **C-like** debug trace macro: `enter!()` and `leave!()`.

# Thanks to [//https://stackoverflow.com/questions/38088067](//https://stackoverflow.com/questions/38088067).

# Example

```rust
use dmacro::*;

fn main() {
    enter!();
    println!("Hello, world!");
    leave!();
}
```
