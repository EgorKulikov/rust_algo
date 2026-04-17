//{"name":"Burglar Movements","group":"CodeChef - START234A","url":"https://www.codechef.com/START234A/problems/BURGMOV","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n3 2\n2 4 5\n5 3\n1 4 3 15 7\n5 6\n1 4 3 15 7\n","output":"4 5 5\n5 15 18 22 22\n22 26 25 25 26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    let mut ltr = Arr2d::new(n + 1, k + 1, 0);
    for pos in (0..n).rev() {
        for rem in 1..=k {
            let mut res = ltr[(pos + 1, rem - 1)];
            res.maxim(ltr[(pos + 1, rem.saturating_sub(2))] + a[pos]);
            ltr[(pos, rem)] = res;
        }
    }
    let mut rtl = Arr2d::new(n + 1, k + 1, 0);
    for pos in 1..=n {
        for rem in 1..=k {
            let mut res = rtl[(pos - 1, rem - 1)];
            res.maxim(rtl[(pos - 1, rem.saturating_sub(2))] + a[pos - 1]);
            rtl[(pos, rem)] = res;
        }
    }
    let ans = Vec::with_gen(n, |i| {
        let mut res = 0;
        for j in 0..n {
            let d = i.abs_diff(j);
            if d <= k {
                res.maxim(ltr[(j, k - d)]);
                res.maxim(rtl[(j + 1, k - d)]);
            }
        }
        res
    });
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
