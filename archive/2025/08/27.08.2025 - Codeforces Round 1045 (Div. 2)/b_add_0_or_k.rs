//{"name":"B. Add 0 or K","group":"Codeforces - Codeforces Round 1045 (Div. 2)","url":"https://codeforces.com/contest/2134/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"8\n3 3\n2 7 1\n4 5\n2 9 16 14\n4 1\n1 2 3 4\n5 2\n5 6 7 8 9\n2 10\n7 9\n1 1000000000\n1\n1 371\n1000000000\n3 6\n1 3 5\n","output":"8 10 10\n7 14 21 14\n2 2 4 4\n9 6 9 12 9\n77 99\n1000000000000000001\n1000000000\n25 15 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);

    let mut i = 2;
    let p = loop {
        if gcd(k, i) == 1 {
            break i;
        }
        i += 1;
    };
    for x in a.iter_mut() {
        while *x % p != 0 {
            *x += k;
        }
    }
    out.print_line(a);
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
