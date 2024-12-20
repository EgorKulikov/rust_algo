//{"name":"Coconut Splat","group":"Kattis","url":"https://open.kattis.com/problems/coconut","interactive":false,"timeLimit":1000,"tests":[{"input":"10 2\n","output":"2\n"},{"input":"10 10\n","output":"7\n"},{"input":"1 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CoconutSplat"}}}

use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_size();
    let n = input.read_size();

    enum State {
        Folded,
        PalmUp,
        PalmDown,
    }
    let mut state = Vec::gen(n, |i, _| (i + 1, State::Folded));
    let mut at = 0;
    while state.len() > 1 {
        at += s - 1;
        at %= state.len();
        match state.remove(at) {
            (id, State::Folded) => {
                state.insert(at, (id, State::PalmUp));
                state.insert(at, (id, State::PalmUp));
            }
            (id, State::PalmUp) => {
                state.insert(at, (id, State::PalmDown));
                at += 1;
            }
            (_, State::PalmDown) => {}
        }
    }
    out.print_line(state[0].0);
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
