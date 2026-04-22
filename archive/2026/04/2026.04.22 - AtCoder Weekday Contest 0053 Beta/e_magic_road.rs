//{"name":"E - Magic Road","group":"AtCoder - AtCoder Weekday Contest 0053 Beta","url":"https://atcoder.jp/contests/awc0053/tasks/awc0053_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 2\n1 2 2\n1 3 3\n2 4 5\n3 4 4\n","output":"22\n"},{"input":"3 1 2\n1 2 5\n","output":"0\n"},{"input":"5 8 3\n1 2 3\n1 3 2\n2 3 4\n2 4 1\n3 4 5\n3 5 2\n4 5 3\n4 2 1\n","output":"63\n"},{"input":"10 15 1000000000\n1 2 100\n2 3 200\n3 4 300\n4 5 400\n5 6 500\n6 7 600\n7 8 700\n8 9 800\n9 10 900\n1 5 10\n5 10 20\n3 7 50\n2 10 1000000000\n1 10 999999999\n7 10 42\n","output":"0\n"},{"input":"2 0 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    type Mod = ModIntF;
    let mut matrix = Matrix::<Mod>::zero(n, n);
    for _ in 0..m {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        let w = input.read_size();
        matrix[u][v] += w;
    }
    out.print_line(matrix.power(k)[(0, n - 1)]);
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
