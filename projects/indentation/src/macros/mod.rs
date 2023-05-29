

#[macro_export]
macro_rules! indent_fmt {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! indent_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}