//{"name":"Cookie Selection","group":"Kattis","url":"https://open.kattis.com/problems/cookieselection","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n2\n3\n4\n#\n#\n#\n#\n","output":"3\n2\n4\n1\n"},{"input":"1\n#\n2\n#\n3\n#\n4\n#\n","output":"1\n2\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CookieSelection"}}}

use algo_lib::collections::divided_set::DividedSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut ds = DividedSet::new(|left, right| {
        if left > right {
            Some(Direction::Right)
        } else if left + 1 < right {
            Some(Direction::Left)
        } else {
            None
        }
    });
    while !input.is_empty() {
        if input.peek() == Some(b'#') {
            input.read_char();
            out.print_line(ds.pop_right_head());
        } else {
            let d = input.read_size();
            ds.insert(d);
        }
    }
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
