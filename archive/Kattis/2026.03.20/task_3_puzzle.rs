//{"name":"3-Puzzle","group":"Kattis","url":"https://open.kattis.com/problems/3puzzle","interactive":false,"timeLimit":1000,"tests":[{"input":"2-\n13\n","output":"3\n"},{"input":"-3\n21\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_char_table(2, 2);

    let mut queue = VecDeque::new();
    queue.push_back(s.clone());
    let mut ans = FxHashMap::default();
    ans.insert(s, 0);

    while let Some(s) = queue.pop_front() {
        if s[(0, 0)] == b'1' && s[(0, 1)] == b'2' && s[(1, 0)] == b'3' {
            out.print_line(ans[&s]);
            return;
        }
        for (r, c) in s.indices() {
            if s[(r, c)] == b'-' {
                let mut t = s.clone();
                t.swap(r, c, r ^ 1, c);
                if !ans.contains_key(&t) {
                    ans.insert(t.clone(), ans[&s] + 1);
                    queue.push_back(t);
                }
                let mut t = s.clone();
                t.swap(r, c, r, c ^ 1);
                if !ans.contains_key(&t) {
                    ans.insert(t.clone(), ans[&s] + 1);
                    queue.push_back(t);
                }
            }
        }
    }

    unreachable!();
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
