//{"name":"T534947 残雪","group":"Luogu","url":"https://www.luogu.com.cn/problem/T534947?contestId=187558","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2 3 5\n3 3 4 6\n5 6 11 13\n10 15 33 22\n10 13 11 11\n","output":"No\nYes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T534947"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let r = input.read_size();
    let n = input.read_size();
    let m = input.read_size();

    let (n, m) = (n.min(m), n.max(m));
    if n == 0 {
        out.print_line(true);
        return;
    }
    if l == 1 {
        out.print_line(false);
        return;
    }
    if (n + 1) / 2 < l && (n != m || r < n) {
        out.print_line(true);
        return;
    }
    let bigs = n.div_ceil(l - 1);
    if (bigs - 1).saturating_mul(r + 1) <= m {
        out.print_line(true);
        return;
    }
    let rem = m - (n - 1);
    out.print_line(rem >= 2 * ((n - 1) / (l - 1)));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
