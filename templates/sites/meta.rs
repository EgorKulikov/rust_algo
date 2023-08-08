use algo_lib::out_line;
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {}

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            $CARET
        }

        fn solve(&mut self) {}

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case));
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();
    solve(&mut input, &pre_calc);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}
