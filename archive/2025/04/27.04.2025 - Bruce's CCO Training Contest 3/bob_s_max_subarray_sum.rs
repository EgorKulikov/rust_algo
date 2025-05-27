//{"name":"Bob's Max Subarray Sum","group":"DMOJ","url":"https://dmoj.ca/problem/oly25practice6","interactive":false,"timeLimit":3000,"tests":[{"input":"5 7\n2 -4 6 -5 5\n1 1 5\n0 1 5 -4\n1 1 5\n0 3 4 -1\n1 1 5\n0 1 3 -1\n1 1 5\n","output":"6\n7\n10\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_long_vec(n);

    for _ in 0..q {
        let command = input.read_int();
        let l = input.read_size() - 1;
        let r = input.read_size();
        match command {
            0 => {
                let x = input.read_long();
                for i in l..r {
                    a[i].maxim(x);
                }
            }
            1 => {
                let mut ans = 0;
                let mut min = 0;
                let mut sum = 0;
                for i in l..r {
                    sum += a[i];
                    min.minim(sum);
                    ans.maxim(sum - min);
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
