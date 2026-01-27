//{"name":"A - Swapping Game","group":"AtCoder - AtCoder Regular Contest 213 (Div. 1)","url":"https://atcoder.jp/contests/arc213/tasks/arc213_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n200 2 1 3\n400 3 1 2\n300 2 3 1\n100 3 2 1\n","output":"600\n"},{"input":"2 1\n0 1\n10000 1\n","output":"10000\n"},{"input":"1 9\n2025 2 4 6 8 7 5 1 9 3\n","output":"0\n"},{"input":"10 5\n2615 5 1 3 4 2\n4275 1 3 2 5 4\n4331 3 1 5 2 4\n1457 3 5 1 2 4\n2222 1 3 2 4 5\n5051 2 4 5 3 1\n1082 2 3 1 5 4\n1579 4 1 5 2 3\n2855 5 1 3 2 4\n5848 2 4 3 1 5\n","output":"12345\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let mut c = Vec::with_capacity(n + 1);
    let mut p = Vec::with_capacity(n + 1);
    c.push(0);
    p.push((0..l).collect());
    for _ in 0..n {
        c.push(input.read_int());
        p.push(input.read_size_vec(l).dec());
    }

    let len = (l * (l - 1) / 2).max(1);
    let can = Arr2d::with_gen(n + 1, len, |i, j| {
        if j == 0 || i + j > n {
            false
        } else {
            let mut ignore = 0;
            let mut q = 0;
            for k in 0..l {
                for m in 0..l {
                    if p[i][k] == p[i + j][m] {
                        break;
                    }
                    if !ignore.is_set(p[i + j][m]) {
                        q += 1;
                    }
                }
                ignore.set_bit(p[i][k]);
            }
            q <= j
        }
    });
    let mut mem = Memoization1d::new(n + 1, |mem, pos| -> (i32, i32) {
        if pos == 0 {
            return (0, 0);
        }
        let mut res = i32::MIN;
        if pos >= len {
            res.maxim(c[pos] + mem.call(pos - len).1);
        }
        for i in 1..len.min(pos + 1) {
            if can[(pos - i, i)] {
                res.maxim(c[pos] + mem.call(pos - i).0);
            }
        }
        (res, res.max(mem.call(pos - 1).1))
    });
    out.print_line(mem.call(n).1);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
