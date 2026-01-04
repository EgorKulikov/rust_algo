//{"name":"B. AND Reconstruction","group":"Universal Cup - GP of Southeastern Europe","url":"https://contest.ucup.ac/contest/2828/problem/16116","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3\n7 3 7 7 3 7\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut ans = n;
    for i in 0..b {
        let mut x = Vec::with_gen(n, |j| a[j].is_set(i));
        if x.copy_count(true) == n {
            continue;
        }
        let pos = x.copy_find(false).unwrap();
        x.rotate_left(pos);
        let mut start = 0;
        for j in 1..n {
            if !x[j] {
                if j - start != 1 {
                    ans.minim(j - start - 1);
                }
                start = j;
            }
        }
        if n - start != 1 {
            ans.minim(n - start - 1);
        }
        for j in 0..n {
            if !x[j] && !x[(j + 1) % n] && !x[(j + 2) % n] {
                ans.minim(1);
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
