## Define **C-like** debug trace macro: `enter!()` and `leave!()`.

## Check crate: [https://crates.io/crates/dmacro](https://crates.io/crates/dmacro)

## Thanks to [https://stackoverflow.com/questions/38088067](https://stackoverflow.com/questions/38088067).

## Example

```rust
use dmacro::*;

fn main() {
    enter!();
    println!("Hello, world!");

    trace!();
    trace!("church11");
    trace!("church21", "church22",);
    trace!("church31", "church32", "church33");
    trace!("church31", "church32", "church33", 1024);
    let y = 2023;
    let m = 9;
    let d = 28;
    trace!(format!("{:04}-{:02}-{:02}", y, m, d));
    trace!(format!("{y:04}-{m:02}-{d:02}"));
    trace!(format!("{y:04}-{m:02}-{d:02}"), "enter");
    trace!(format!("{y:04}-{m:02}-{d:02}"), "leave");

    leave!();
}
```

## More

   - [https://crates.io/crates/trace](https://crates.io/crates/trace)

