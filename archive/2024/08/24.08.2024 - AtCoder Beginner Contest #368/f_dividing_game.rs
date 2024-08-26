//{"name":"F - Dividing Game","group":"AtCoder - Hitachi Vantara Programming Contest 2024（AtCoder Beginner Contest 368）","url":"https://atcoder.jp/contests/abc368/tasks/abc368_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 4\n","output":"Anna\n"},{"input":"4\n2 3 4 6\n","output":"Bruno\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDividingGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::primes::factorize::all_divisors;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let d = all_divisors::<usize>(100_001, false);
    let mut nim = vec![0; 100_001];
    for i in 2..=100_000 {
        let mut options = HashSet::new();
        for &d in &d[i] {
            if d != i {
                options.insert(nim[d]);
            }
        }
        for j in 0.. {
            if !options.contains(&j) {
                nim[i] = j;
                break;
            }
        }
    }
    let mut ans = 0;
    for i in a {
        ans ^= nim[i];
    }
    out.print_line(if ans == 0 { "Bruno" } else { "Anna" });
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
