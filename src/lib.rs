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

#[macro_export]
macro_rules! enter {
    () => {
        $crate::trace!("enter")
    };
}

#[macro_export]
macro_rules! leave {
    () => {
        $crate::trace!("leave")
    };
}
