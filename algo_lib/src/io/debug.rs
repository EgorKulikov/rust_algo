#[macro_export]
macro_rules! debug {
    ($($expression: expr$(,)?)+) => {
        #[cfg(debug_assertions)]
        {
            use std::io::Write;
            let mut err = $crate::io::output::Output::delegate(std::io::stderr());
            $(
                eprint!("{} = ", stringify!($expression));
                err.print(&$expression);
                err.flush();
                eprint!("; ");
            )+
            eprintln!();
        }
    };
}
