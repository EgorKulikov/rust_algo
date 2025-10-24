//{"name":"B - Sort Permutation","group":"AtCoder - AtCoder Regular Contest 204 (Div. 1)","url":"https://atcoder.jp/contests/arc204/tasks/arc204_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 6 5 3 2 4\n","output":"2\n"},{"input":"1 1\n1\n","output":"0\n"},{"input":"4 6\n10 24 3 4 8 14 5 2 22 9 21 1 15 6 13 23 18 12 7 17 19 16 20 11\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_size_vec(n * k).dec();

    let mut done = BitSet::new(n * k);
    let mut ans = 0;
    for i in 0..n * k {
        if done[i] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut j = i;
        while !done[j] {
            done.set(j);
            cycle.push(j);
            j = p[j];
        }
        let mut t = vec![Vec::new(); cycle.len()];
        for i in cycle.indices() {
            for j in i + 1..cycle.len() {
                if cycle[i].abs_diff(cycle[j]) % n == 0 {
                    t[i].push(j);
                }
            }
        }
        let mut mem =
            Memoization2d::new(cycle.len() + 1, cycle.len() + 1, |mem, from, to| -> i32 {
                if from == to {
                    0
                } else {
                    let mut res = mem.call(from + 1, to);
                    for i in t[from].copy_iter() {
                        if i >= to {
                            break;
                        }
                        res.maxim(mem.call(from + 1, i) + mem.call(i, to) + 1);
                    }
                    res
                }
            });
        ans += mem.call(0, cycle.len());
    }
    out.print_line(ans);
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
