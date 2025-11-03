//{"name":"problem_c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();

    let mut f = vec![0, 1];
    let mut options = vec![(1, 1)];
    while f[Back(0)] <= k {
        f.push(f[Back(0)] + f[Back(1)]);
        if k % f[Back(0)] == 0 {
            options.push((f[Back(0)], f.len() - 1));
        }
    }
    let mut mem = Memoization2::new(|mem, len: usize, ways: i64| -> Option<usize> {
        if len == 0 {
            if ways == 1 {
                Some(0)
            } else {
                None
            }
        } else {
            for (times, l) in options.copy_iter() {
                if l < len && ways % times == 0 && mem.call(len - l - 1, ways / times).is_some() {
                    return Some(l);
                }
            }
            None
        }
    });
    if let Some(len) = mem.call(n + 1, k) {
        let mut ans = Vec::new();
        if len != n {
            ans.push(len + 1);
            let mut at = len + 2;
            let mut ways = k / f[len];
            loop {
                let len = mem.call(n + 2 - at, ways).unwrap();
                if at + len < n {
                    ans.push(at + len);
                    at += len + 1;
                    ways /= f[len];
                } else {
                    assert_eq!(at + len, n + 1);
                    break;
                }
            }
        }
        out.print_line(ans.len());
        out.print_line(ans);
    } else {
        out.print_line(-1);
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
