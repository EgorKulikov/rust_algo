//{"name":"B - Orchard Harvest","group":"AtCoder - AtCoder Weekday Contest 0050 Beta","url":"https://atcoder.jp/contests/awc0050/tasks/awc0050_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n5 2\n3 5\n10 4\n","output":"13\n"},{"input":"2 7\n4 3\n2 1\n","output":"6\n"},{"input":"8 15\n12 3\n7 10\n25 4\n9 2\n14 7\n30 6\n5 1\n18 5\n","output":"82\n"},{"input":"20 60\n45 6\n120 15\n78 9\n33 4\n200 25\n17 3\n96 8\n150 20\n64 7\n89 11\n210 30\n58 5\n132 12\n41 10\n73 6\n185 18\n27 2\n99 13\n160 16\n52 14\n","output":"1143\n"},{"input":"1 1000000000000000000\n1000000000 1000000000\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let ab = input.read_size_pair_vec(n);

    let mut vars = Vec::new();
    for i in 0..n {
        let (a, b) = ab[i];
        vars.push((b, a / b));
        vars.push((a % b, 1));
    }
    vars.sort();
    vars.reverse();
    let mut ans = 0;
    for (val, qty) in vars {
        let take = qty.min(k);
        ans += take * val;
        k -= take;
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
