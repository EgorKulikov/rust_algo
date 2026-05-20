//{"name":"B - Flipping Cards","group":"AtCoder - AtCoder Weekday Contest 0073 Beta","url":"https://atcoder.jp/contests/awc0073/tasks/awc0073_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n1 5\n3 2\n4 1\n","output":"12\n"},{"input":"3 0\n2 5\n3 1\n4 3\n","output":"9\n"},{"input":"8 3\n10 20\n15 5\n8 12\n6 6\n20 25\n3 1\n7 18\n9 4\n","output":"104\n"},{"input":"15 5\n12 30\n25 10\n8 15\n50 45\n33 40\n19 2\n7 55\n44 44\n16 28\n5 90\n61 20\n38 42\n14 9\n27 35\n10 10\n","output":"540\n"},{"input":"1 1\n3 7\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let fb = input.read_int_pair_vec(n);

    let mut ans = 0;
    let mut add = Vec::with_capacity(n);
    for (f, b) in fb {
        ans += f;
        add.push(b - f);
    }
    add.sort();
    ans += add[n - k..].iter().sum::<i32>();
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
