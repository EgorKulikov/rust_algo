type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    $CARET
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();
    solve(&mut input, &pre_calc);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}
