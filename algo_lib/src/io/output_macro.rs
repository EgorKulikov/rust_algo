#[macro_export]
macro_rules! output {
    ($output: expr, $($arg:tt)*) => {
        $output.print_line(format!($($arg)*));
    };
}
