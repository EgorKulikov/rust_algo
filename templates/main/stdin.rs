    let mut sin = std::io::stdin();
    let input = if $INTERACTIVE {
        crate::io::input::Input::new_with_size(&mut sin, 1)
    } else {
        crate::io::input::Input::new(&mut sin)
    };
