//{"name":"D. Inversion Value of a Permutation","group":"Codeforces - Educational Codeforces Round 183 (Rated for Div. 2)","url":"https://codeforces.com/contest/2145/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n4 5\n5 10\n5 0\n6 8\n3 1\n","output":"3 1 4 2\n5 4 3 2 1\n1 2 3 4 5\n2 3 5 6 1 4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut mem = Memoization3d::new(
        31,
        30 * 29 / 2 + 1,
        31,
        |mem, n, k, a| -> Option<Vec<usize>> {
            if n == 0 {
                if k == 0 {
                    Some(vec![])
                } else {
                    None
                }
            } else {
                if let Some(mut res) = mem.call(n - 1, k, a + 1) {
                    res.push(n);
                    return Some(res);
                }
                for i in 0..n - 1 {
                    let add = (i + 1) * (n - i - 1 + a);
                    if k >= add {
                        let rem = k - add;
                        for j in 0..=rem {
                            if let Some(mut prefix) = mem.call(i, j, 1) {
                                if let Some(suffix) = mem.call(n - i - 1, rem - j, a) {
                                    prefix.push(n);
                                    for x in suffix {
                                        prefix.push(x + i);
                                    }
                                    return Some(prefix);
                                }
                            }
                        }
                    }
                }
                None
            }
        },
    );

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let k = input.read_size();
        if let Some(ans) = mem.call(n, k, 0) {
            out.print_line(ans);
        } else {
            out.print_line(0);
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
