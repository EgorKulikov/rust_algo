    let mut sin = std::io::stdin();
    let input = if $INTERACTIVE {
        algo_lib::io::input::Input::new_with_size(&mut sin, 1)
    } else {
        algo_lib::io::input::Input::new(&mut sin)
    };
