//{"name":"E - Minimum Cost Guard Plan","group":"AtCoder - AtCoder Weekday Contest 0050 Beta","url":"https://atcoder.jp/contests/awc0050/tasks/awc0050_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n4 6 5 11\n1 1 0\n0 1 1\n1 0 0\n1 1 1\n","output":"10\n"},{"input":"4 4\n5 2 7 4\n1 0 0 0\n0 1 1 0\n1 0 1 0\n0 1 0 0\n","output":"-1\n"},{"input":"7 6\n8 5 6 7 4 9 3\n1 0 1 0 0 0\n0 1 0 1 0 0\n0 0 1 1 1 0\n1 1 0 0 0 1\n0 0 0 1 0 1\n1 0 0 0 1 0\n0 1 0 0 1 1\n","output":"13\n"},{"input":"12 10\n14 9 20 11 8 7 13 6 15 10 12 5\n1 0 1 0 0 1 0 0 0 0\n0 1 0 1 0 0 1 0 0 0\n1 1 0 0 1 0 0 1 0 0\n0 0 1 1 0 1 0 0 1 0\n1 0 0 0 1 0 1 0 0 1\n0 1 1 0 0 0 0 1 1 0\n0 0 0 1 1 1 0 0 0 1\n1 0 0 0 0 0 1 1 0 0\n0 1 0 1 0 1 0 1 0 1\n0 0 1 0 1 0 1 0 1 0\n1 0 0 1 0 0 1 0 1 0\n0 1 0 0 1 0 0 1 0 1\n","output":"22\n"},{"input":"1 1\n1000000000\n1\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
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
    let c = input.read_long_vec(n);
    let e = input.read_int_table(n, m);

    let mask = Vec::with_gen(n, |i| {
        let mut res = 0;
        for j in 0..m {
            if e[(i, j)] == 1 {
                res.set_bit(j);
            }
        }
        res
    });
    let mut ans = vec![None; 1 << m];
    ans[0] = Some(0);
    for i in 0..n {
        for j in usize::iter_all(m).rev() {
            if let Some(cost) = ans[j] {
                ans[j | mask[i]].minim(cost + c[i]);
            }
        }
    }
    out.print_line(ans[Back(0)]);
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
