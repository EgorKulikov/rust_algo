//{"name":"G. Mixing MEXes","group":"Codeforces - Codeforces Round 1074 (Div. 4)","url":"https://codeforces.com/contest/2185/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2\n1 0\n2 1 2\n3\n1 1\n2 2 3\n3 4 5 6\n5\n4 1 7 8 10\n2 5 6\n2 0 7\n2 6 6\n2 6 8\n2\n1 3\n3 0 1 2\n2\n6 0 0 1 2 2 3\n3 0 2 3\n10\n1 0\n9 7 8 0 1 5 6 4 3 2\n8 4 3 8 6 2 5 0 1\n7 2 3 0 1 0 4 0\n2 3 1\n9 2 0 5 4 1 3 0 0 0\n7 6 3 2 4 1 8 0\n5 3 2 4 1 0\n4 0 3 1 1\n3 0 3 2\n","output":"6\n0\n50\n8\n43\n19202\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::{qty, DefaultHashMap};
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_vec::<Vec<usize>>(n);

    let mut val = Vec::new();
    let mut delta = DefaultHashMap::new(0);
    let mut sum = 0;
    for i in 0..n {
        let q = qty(&a[i]);
        let mut first = None;
        for j in 0.. {
            if q[j] == 0 {
                if let Some(first) = first {
                    delta[first] += j - first;
                    break;
                } else {
                    val.push(j);
                    sum += j;
                    first = Some(j);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        let q = qty(&a[i]);
        for j in a[i].copy_iter() {
            let we = if j < val[i] && q[j] == 1 { j } else { val[i] };
            ans += (we + sum - val[i]) * (n - 1) + delta[j];
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
