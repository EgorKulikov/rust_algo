fn solve(input: &mut Input, _test_case: usize) {
    $CARET
}

pub(crate) fn run(mut input: Input) -> bool {
    $INVOKE
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}
