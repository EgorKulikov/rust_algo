//{"name":"E - The Adventurer's Journey","group":"AtCoder - AtCoder Weekday Contest 0052 Beta","url":"https://atcoder.jp/contests/awc0052/tasks/awc0052_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2 10\n5 3 7\n1 2 8\n2 3 6\n","output":"11\n"},{"input":"3 1 5\n0 0 0\n1 2 3\n","output":"-1\n"},{"input":"5 6 10\n5 100 3 2 8\n1 2 12\n1 3 5\n2 3 8\n2 5 10\n3 4 3\n4 5 3\n","output":"103\n"},{"input":"8 10 50\n10 20 30 40 50 15 25 100\n1 2 30\n1 3 40\n1 6 35\n2 3 25\n3 4 20\n3 8 80\n4 5 15\n5 8 10\n6 7 20\n7 8 30\n","output":"200\n"},{"input":"2 1 0\n1000 1000\n1 2 1000\n","output":"1000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let f = input.read_int();
    let r = input.read_int_vec(n);
    let edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut ans = Arr2d::new(1 << n, n, None);
    ans[(1, 0)] = Some(f + r[0]);
    let mut res = None;
    for i in usize::iter_all(n) {
        loop {
            let mut update = false;
            for (a, b, c) in edges.copy_iter() {
                for (a, b) in [(a, b), (b, a)] {
                    if let Some(cur) = ans[(i, a)] {
                        if cur >= c {
                            if i.is_set(b) {
                                update |= ans[(i, b)].maxim(cur - c);
                            } else {
                                ans[(i.with_bit(b), b)].maxim(cur - c + r[b]);
                            }
                        }
                    }
                }
            }
            if !update {
                break;
            }
        }
        if let Some(v) = ans[(i, n - 1)] {
            res.maxim(v);
        }
    }
    out.print_line(res);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
