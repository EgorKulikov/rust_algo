    let mut in_file = std::fs::File::open("$IN_FILE").unwrap();
    let input = algo_lib::io::input::Input::new(&mut in_file);
