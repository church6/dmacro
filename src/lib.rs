macro_rules! _function {
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

macro_rules! _trace {
    ($x:expr) => {
        println!(
            "[{}][{}] {} (in {} [{}:{}:{}])",
            chrono::Local::now(),
            $x,
            _function!(),
            module_path!(),
            file!(),
            line!(),
            column!()
        );
    };
}

#[macro_export]
macro_rules! enter {
    () => {
        _trace!("enter")
    };
}

#[macro_export]
macro_rules! leave {
    () => {
        _trace!("leave")
    };
}
