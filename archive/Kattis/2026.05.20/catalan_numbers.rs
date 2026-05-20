//{"name":"Catalan Numbers","group":"Kattis","url":"https://open.kattis.com/problems/catalan","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n50\n","output":"1\n2\n5\n14\n42\n1978261657756160653623774456\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::unsigned_big_int::UBigInt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut cur = UBigInt::one();
    let mut ans = Vec::with_capacity(5001);
    ans.push(cur.clone());
    ans.push(cur.clone());
    for n in 2..=5000 {
        cur *= 2 * n - 1;
        cur *= 2 * n;
        cur /= n;
        cur /= n + 1;
        ans.push(cur.clone());
    }

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        out.print_line(&ans[n]);
    }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
