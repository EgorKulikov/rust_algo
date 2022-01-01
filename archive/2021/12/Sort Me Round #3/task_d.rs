//{"name":"task_d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_d"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::hungarian_algorithm::hungarian_algorithm;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n).dec_by_one();
    let b = input.read_usize_vec(n).dec_by_one();

    let mut mat = Arr2d::new(512, 512, 0i64);
    for (i, j) in a.into_iter().zip(b.into_iter()) {
        mat[(i, j)] -= 1;
    }
    out_line!(-hungarian_algorithm(&mat));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
