    let mut stdout = std::io::stdout();
    let output = if $INTERACTIVE {
        io::output::Output::new_with_auto_flush(&mut stdout)
    } else {
        io::output::Output::new(&mut stdout)
    };
