//{"name":"Echo","group":"CodeChef - START235A","url":"https://www.codechef.com/START235A/problems/P6235","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n7 2 9\n3 1\n6 4 7\n4 2\n1 5 3 6\n5 3\n8 6 4 2 9\n","output":"2\n2\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size() - 1;
    let a = input.read_unsigned_vec(n);

    if p == 0 {
        let mut ans = None;
        let mut cur = 0;
        for i in 0..n - 1 {
            cur ^= a[i];
            ans.minim(cur);
        }
        out.print_line(ans);
        return;
    }
    if p == n - 1 {
        let mut ans = None;
        let mut cur = 0;
        for i in (1..n).rev() {
            cur ^= a[i];
            ans.minim(cur);
        }
        out.print_line(ans);
        return;
    }
    let mut v = Vec::new();
    let mut cur = 0;
    v.push(0);
    for i in 1..n - 1 {
        cur ^= a[i];
        v.push(cur);
    }
    v.sort();
    let mut ans = None;
    for (a, b) in v.consecutive_iter_copy() {
        ans.minim(a ^ b);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
