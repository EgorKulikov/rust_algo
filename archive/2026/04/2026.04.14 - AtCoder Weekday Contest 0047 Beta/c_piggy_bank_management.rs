//{"name":"C - Piggy Bank Management","group":"AtCoder - AtCoder Weekday Contest 0047 Beta","url":"https://atcoder.jp/contests/awc0047/tasks/awc0047_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n10 20 30 40 50\n2 3\n1 2 4 5\n2 3\n2 1\n","output":"30\n35\n10\n"},{"input":"8 7\n100 200 300 400 500 600 700 800\n1 1 5 10\n1 3 8 20\n2 3\n2 6\n1 1 8 100\n2 1\n2 8\n","output":"330\n620\n210\n920\n"},{"input":"10 10\n0 1000000000 500 0 999999999 1 2 3 4 5\n1 1 10 10000\n2 2\n1 5 5 3\n2 5\n1 1 1 1\n2 1\n1 3 7 100\n2 4\n2 10\n2 7\n","output":"1000010000\n1000010002\n10001\n10100\n10005\n10102\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    let mut ft = FenwickTree::new(n + 1);
    let mut was = 0;
    for i in 0..n {
        ft.add(i, a[i] - was);
        was = a[i];
    }
    for _ in 0..q {
        let mode = input.read_int();
        match mode {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_long();
                ft.add(l, x);
                ft.add(r, -x);
            }
            2 => {
                let p = input.read_size() - 1;
                out.print_line(ft.get(..=p));
            }
            _ => unreachable!(),
        }
    }
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
