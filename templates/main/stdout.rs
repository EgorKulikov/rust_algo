    let mut stdout = std::io::stdout();
    let output = if $INTERACTIVE {
        crate::io::output::Output::new_with_auto_flush(&mut stdout)
    } else {
        crate::io::output::Output::new(&mut stdout)
    };
