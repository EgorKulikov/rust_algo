//{"name":"S5 - World Tree","group":"DMOJ - OTHS Coding Competition 3 (Mock CCC)","url":"https://dmoj.ca/problem/othscc3p5","interactive":false,"timeLimit":4000,"tests":[{"input":"5 5\n1 1 2 2\n1 2 3 4 5\n1 1 6\n2 3\n2 4\n1 2 1\n2 5\n","output":"2.5\n0.25\n0.75\n"},{"input":"5 6\n3 1 1 3\n1 1 1 1 1\n1 1 999\n2 1\n2 2\n2 3\n2 4\n2 5\n","output":"1\n1\n1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let p = input.read_size_vec(n - 1).dec();
    let a = input.read_real_vec(n);

    let mut val = vec![Real(0.); n];
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let x = input.read_size() - 1;
                let v = input.read_real();
                val[x] += v;
            }
            2 => {
                let mut x = input.read_size() - 1;
                let mut anc = Vec::new();
                let ox = x;
                while anc.len() < 50 && x != 0 {
                    x = p[x - 1];
                    anc.push(x);
                }
                let mut down = Real(0.);
                for i in anc.copy_rev() {
                    down += val[i];
                    down -= a[i];
                    down.maxim(Real(0.));
                    down /= 2;
                }
                out.print_line((val[ox] + down).min(a[ox]));
            }
            _ => unreachable!(),
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
