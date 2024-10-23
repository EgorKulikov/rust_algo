//{"name":"Unmedian","group":"CodeChef - START157A","url":"https://www.codechef.com/START157A/problems/MEDIANT","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n3 4 1 4 5\n3\n3 2 1\n6\n1 4 2 4 5 3\n","output":"2\n2 4\n1 3\n-1\n2\n2 4\n1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Unmedian"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::iter::repeat;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let last_max_pos = a
        .iter()
        .enumerate()
        .max_by_key(|&(i, &x)| (x, i))
        .unwrap()
        .0;
    let last_min_pos = a
        .iter()
        .enumerate()
        .min_by_key(|&(i, &x)| (x, Reverse(i)))
        .unwrap()
        .0;

    if last_min_pos > last_max_pos {
        out.print_line(-1);
    } else {
        out.print_line(n - 2);
        out.print_per_line_iter(repeat((1, 3)).take(n - 2));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
