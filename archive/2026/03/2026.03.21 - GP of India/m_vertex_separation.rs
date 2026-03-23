//{"name":"M. Vertex Separation","group":"Universal Cup - GP of India","url":"https://contest.ucup.ac/contest/3516/problem/17418","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 1 1\n3 2 0\n4 5 4\n","output":"-1\n1 2\n2 3\n1 2\n2 3\n3 4\n1 4\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    if k != n && k != 0 {
        if m >= n + 1 {
            let triangles = m - (n - 1);
            let k = n - k;
            if k + triangles * 2 <= n {
                out.print_line((1, 2));
                out.print_line((2, 3));
                out.print_line((1, 3));
                for i in 0..k - 1 {
                    out.print_line((3 + i, 4 + i));
                }
                for i in 0..triangles - 1 {
                    out.print_line((2 + k, 3 + k + i * 2));
                    out.print_line((2 + k, 4 + k + i * 2));
                    out.print_line((3 + k + i * 2, 4 + k + i * 2));
                }
                for i in k + triangles * 2..n {
                    out.print_line((i, i + 1));
                }
                return;
            }
        }
        out.print_line(-1);
        return;
    }
    if k == 0 {
        if m > n {
            out.print_line(-1);
            return;
        }
        for i in 1..n {
            out.print_line((i, i + 1));
        }
        if m == n {
            assert!(n >= 3);
            out.print_line((1, 3));
        }
        return;
    }
    if m == n - 1 || n <= 3 {
        out.print_line(-1);
        return;
    }
    let mut used = FxHashSet::default();
    for i in 1..n {
        out.print_line((i, i + 1));
        used.insert((i, i + 1));
    }
    out.print_line((1, n));
    used.insert((1, n));
    let mut left = m - n;
    for i in 1..=n {
        for j in i + 1..=n {
            if left == 0 {
                return;
            }
            if used.contains(&(i, j)) {
                continue;
            }
            out.print_line((i, j));
            left -= 1;
        }
    }
    assert_eq!(left, 0);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
