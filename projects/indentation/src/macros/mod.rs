

/// A macro to create a `String` from a format string.
#[macro_export]
macro_rules! indent_fmt {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// A macro to write to a `Formatter` from a format string.
#[macro_export]
macro_rules! indent_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}