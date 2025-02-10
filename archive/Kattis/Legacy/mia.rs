//{"name":"Mia","group":"Kattis","url":"https://open.kattis.com/problems/mia","interactive":false,"timeLimit":1000,"tests":[{"input":"1 2 1 3\n3 3 2 1\n6 6 4 4\n6 5 1 1\n4 2 2 4\n0 0 0 0\n","output":"Player 1 wins.\nPlayer 2 wins.\nPlayer 1 wins.\nPlayer 2 wins.\nTie.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Mia"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Ordering;

use algo_lib::misc::test_type::TestType;
use algo_lib::when;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p1 = input.read_int_vec(2);
    let p2 = input.read_int_vec(2);
    if p1[0] == 0 {
        return;
    }

    fn score(mut p: Vec<i32>) -> i32 {
        p.sort_unstable();
        let val = p[1] * 10 + p[0];
        when! {
            val == 21 => 1000,
            p[0] == p[1] => val * 10,
            else => val,
        }
    }

    out.print_line(match score(p1).cmp(&score(p2)) {
        Ordering::Less => "Player 2 wins.",
        Ordering::Equal => "Tie.",
        Ordering::Greater => "Player 1 wins.",
    });
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
