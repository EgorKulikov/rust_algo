//{"name":"D - Virus Testing and Infected Terminals","group":"AtCoder - AtCoder Weekday Contest 0052 Beta","url":"https://atcoder.jp/contests/awc0052/tasks/awc0052_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 1 2 1\n2 2 3 0\n","output":"1\n"},{"input":"5 4\n3 1 2 3 1\n2 4 5 1\n2 1 4 0\n2 2 5 1\n","output":"2\n"},{"input":"8 5\n4 1 2 3 4 1\n4 5 6 7 8 1\n4 1 3 5 7 1\n4 2 4 6 8 0\n4 3 4 7 8 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
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
    let mut s = Vec::with_capacity(m);
    let mut r = Vec::with_capacity(m);
    for _ in 0..m {
        let k = input.read_size();
        s.push(input.read_vec(k).dec());
        r.push(input.read_int() == 1);
    }

    let mut ans = n + 1;
    'outer: for i in usize::iter_all(n) {
        if i.count_ones() as usize >= ans {
            continue;
        }
        for j in 0..m {
            let mut infected = false;
            for x in s[j].copy_iter() {
                if i.is_set(x) {
                    infected = true;
                    break;
                }
            }
            if infected != r[j] {
                continue 'outer;
            }
        }
        ans = i.count_ones() as usize;
    }
    out.print_line(ans);
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
