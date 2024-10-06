use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    $CARET
    drop(input);
    out.print_line((format!("Case #{}:", test_case),));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}
