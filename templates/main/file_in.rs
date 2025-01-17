    let in_file = std::fs::File::open("$IN_FILE").unwrap();
    let input = algo_lib::io::input::Input::file(in_file);