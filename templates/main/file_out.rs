    let mut out_file = std::fs::File::create("$OUT_FILE").unwrap();
    let output = io::output::Output::new(&mut out_file);
