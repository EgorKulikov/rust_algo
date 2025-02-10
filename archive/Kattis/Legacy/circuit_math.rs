//{"name":"Circuit Math","group":"Kattis","url":"https://open.kattis.com/problems/circuitmath","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nT F T F\nA B * C D + - +\n","output":"F\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CircuitMath"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let val = input
        .iter::<u8>()
        .take(n)
        .map(|x| x == b'T')
        .collect::<Vec<_>>();

    let mut stack = Vec::new();
    for c in input.iter::<u8>() {
        match c {
            b'*' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a && b);
            }
            b'+' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a || b);
            }
            b'-' => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            b'A'..=b'Z' => {
                stack.push(val[(c - b'A') as usize]);
            }
            _ => unreachable!(),
        }
    }
    out.print_line(if stack[Back(0)] { "T" } else { "F" });
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
