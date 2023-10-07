///
/// ## Define **C-like** debug trace macro: `function!()`.
///
/// ```rust
/// use dmacro::*;
/// fn foo(n: usize) {
///     println!("{}", function!());
///
///     // ...
/// }
///
/// foo(20080512usize);
/// ```
///
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            // 'church
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

///
/// ## Define **C-like** debug trace macro: `trace!()`.
///
/// ```rust
/// use dmacro::*;
/// fn foo(n: usize) {
///     trace!(format!("n = {}", n));
///
///     // ...
///     trace!("ur lucky!");
/// }
///
/// foo(20080512usize);
/// ```
///
/// ## Define **C-like** debug trace macro: `enter!()` and `leave!()`.
///
/// ## Check crate: [https://crates.io/crates/dmacro](https://crates.io/crates/dmacro)
///
/// ## Thanks to [https://stackoverflow.com/questions/38088067](https://stackoverflow.com/questions/38088067).
///
/// ## Example
///
/// ```rust
/// use dmacro::*;
///
/// fn main() {
///     enter!();
///     println!("Hello, world!");
///
///     trace!();
///     trace!("church11");
///     trace!("church21", "church22",);
///     trace!("church31", "church32", "church33");
///     trace!("church31", "church32", "church33", 1024);
///     let y = 2023;
///     let m = 9;
///     let d = 28;
///     trace!(format!("{:04}-{:02}-{:02}", y, m, d));
///     trace!(format!("{y:04}-{m:02}-{d:02}"));
///     trace!(format!("{y:04}-{m:02}-{d:02}"), "enter");
///     trace!(format!("{y:04}-{m:02}-{d:02}"), "leave");
///
///     leave!();
/// }
/// ```
///
/// ## More
///
///    - [https://crates.io/crates/trace](https://crates.io/crates/trace)
///
#[macro_export]
macro_rules! trace {
    () => {
        println!(
            "[{}][{}:{}:{}]{}/{}",
            chrono::Local::now(),
            file!(),
            line!(),
            column!(),
            module_path!(),
            $crate::function!()
        )
    };

    ($val:expr) => {
        println!(
            "[{}][{}:{}:{}]{}/{} {}",
            chrono::Local::now(),
            file!(),
            line!(),
            column!(),
            module_path!(),
            $crate::function!(),
            $val
        )
    };

    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                println!(
                    "[{}][{}:{}:{}]{}/{} {}",
                    chrono::Local::now(),
                    file!(),
                    line!(),
                    column!(),
                    module_path!(),
                    $crate::function!(),
                    &tmp
                );
                tmp
            }
        }
    };

    ($($val:expr),+ $(,)?) => {
        ($($crate::trace!($val)),+,)
    };
}

///
/// ## Define **C-like** debug trace macro: `enter!()`.
///
/// ```rust
/// use dmacro::*;
/// fn foo(n: usize) {
///     enter!();
///     // ...
///     leave!();
/// }
///
/// foo(20080512usize);
/// ```
///
#[macro_export]
macro_rules! enter {
    () => {
        $crate::trace!("enter")
    };
}

///
/// ## Define **C-like** debug trace macro: `leave!()`.
///
/// ```rust
/// use dmacro::*;
/// fn foo(n: usize) {
///     enter!();
///     // ...
///     leave!();
/// }
///
/// foo(20080512usize);
/// ```
///
#[macro_export]
macro_rules! leave {
    () => {
        $crate::trace!("leave")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        enter!();
        leave!();
    }

    #[test]
    fn test2() {
        trace!("enter");
        trace!("leave");
    }

    #[test]
    fn test3() {
        trace!();
        trace!("");
        trace!("OK");
        trace!(20080512usize);
    }
}
