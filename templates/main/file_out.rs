    let out_file = std::fs::File::create("$OUT_FILE").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);