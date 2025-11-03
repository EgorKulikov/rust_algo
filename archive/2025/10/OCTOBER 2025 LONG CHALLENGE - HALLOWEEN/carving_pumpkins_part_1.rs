//{"name":"carving_pumpkins_part_1","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let w = input.read_int();
    let h = input.read_int();
    let e = input.read_int();
    let m = input.read_int();

    output!(out, "Case #{}:", test_case);
    'outer: for i in 1..=h / 2 {
        for j in 1..=w / 2 {
            if (i - 1) * (j - 1) == e
                && Rational::new(i * i, h * h) + Rational::new(j * j, w * w) < Rational::new(1, 4)
            {
                out.print_line((-j, i, -1, 1));
                out.print_line((1, i, j, 1));
                break 'outer;
            }
        }
    }
    'outer: for i in 1..=h / 2 {
        for j in 1..=w {
            if (i - 1) * j == m {
                let x = (j + 1) / 2;
                if Rational::new(i * i, h * h) + Rational::new(x * x, w * w) < Rational::new(1, 4) {
                    out.print_line((-x, -1, j - x, -i));
                    break 'outer;
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
