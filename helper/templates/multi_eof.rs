    let mut i = 1usize;
    while input.peek().is_some() {
        solve(&mut input, i);
        i += 1;
    }
