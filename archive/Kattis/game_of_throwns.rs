//{"name":"Game of Throwns","group":"Kattis","url":"https://open.kattis.com/problems/throwns","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n8 -2 3 undo 2\n","output":"3\n"},{"input":"5 10\n7 -3 undo 1 4 3 -9 5 undo 2 undo 1 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GameOfThrowns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let k = input.read_size();

    let mut ans = 0;
    let mut stack = Vec::new();
    for _ in 0..k {
        if input.peek() == Some(b'u') {
            input.read_str();
            let q = input.read_size();
            for _ in 0..q {
                let x = stack.pop().unwrap();
                ans -= x;
                ans %= n;
                ans += n;
                ans %= n;
            }
        } else {
            let x = input.read_int();
            ans += x;
            ans %= n;
            ans += n;
            ans %= n;
            stack.push(x);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
