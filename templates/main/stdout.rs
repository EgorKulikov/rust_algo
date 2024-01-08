    let mut stdout = std::io::stdout();
    let output = if $INTERACTIVE {
        algo_lib::io::output::Output::new_with_auto_flush(&mut stdout)
    } else {
        algo_lib::io::output::Output::new(&mut stdout)
    };
