use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    $CARET
    out_line!(format!("Case #{}:", test_case));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}
