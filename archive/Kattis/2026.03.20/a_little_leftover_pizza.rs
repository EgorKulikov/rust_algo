//{"name":"A Little Leftover Pizza","group":"Kattis","url":"https://open.kattis.com/problems/alittleleftoverpizza","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nS 0\nM 5\nL 0\n","output":"1\n"},{"input":"3\nS 3\nS 4\nS 2\n","output":"2\n"},{"input":"4\nS 1\nM 1\nM 3\nL 1\n","output":"3\n"},{"input":"4\nL 6\nM 2\nM 6\nL 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_map::ArrayMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pizzas = input.read_vec::<(u8, usize)>(n);

    let mut ans = ArrayMap::<_, usize>::new(b'L'..=b'S');
    let ratio = ArrayMap::with_gen(b'L'..=b'S', |c| match c {
        b'L' => 12,
        b'M' => 8,
        b'S' => 6,
        _ => 0,
    });
    for (c, r) in pizzas {
        ans[c] += r;
    }
    let mut res = 0;
    for c in b'L'..=b'S' {
        if ratio[c] > 0 {
            res += ans[c].upper_div(ratio[c]);
        }
    }
    out.print_line(res);
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
