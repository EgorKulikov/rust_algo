//{"name":"T507043 「LAOI-8」Change","group":"Luogu","url":"https://www.luogu.com.cn/problem/T507043?contestId=187787","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2 3 4 5\n1 2 3 5 4\n","output":"1\n"},{"input":"5\n1 2 3 5 4\n1 3 4 2 5\n","output":"1\n"},{"input":"5\n1 4 3 2 5\n1 2 3 4 5\n","output":"1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let Compressed { arrs: [a, b], .. } = compress([&a, &b]);
    let b_inv = b.inv();

    let mut g = 0;
    for i in 0..n {
        let cur = i.abs_diff(b_inv[a[i]]);
        g = gcd(g, cur);
    }
    for i in 1..=n {
        if g % i == 0 {
            out.print_line(i);
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
