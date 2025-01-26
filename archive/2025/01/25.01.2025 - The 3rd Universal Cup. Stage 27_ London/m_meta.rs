//{"name":"M. Meta","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8623","interactive":false,"timeLimit":1000,"tests":[{"input":"13\nAplusB -1 20 -1\nTheBestWife 80 90 60\nCardinality 40 50 30\n3D 40 -1 70\nEqualStrings 25 15 20\nFastTreeQueries 120 -1 40\nGeoSharding 25 20 30\nHaveYouSeenThisSubarray 80 90 60\nInteractiveCasino 50 20 30\nJigsawPuzzle 40 50 80\nKnapsack -1 40 200\nLondonUnderground -1 200 40\nMeta 5 7 10\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let tasks: Vec<(Str, i32, i32, i32)> = input.read_vec(n);

    let mut times = Vec::new();
    for (_, a, b, c) in tasks {
        let mut cur = None;
        if a != -1 {
            cur.minim(a);
        }
        if b != -1 {
            cur.minim(b);
        }
        if c != -1 {
            cur.minim(c);
        }
        if let Some(cur) = cur {
            times.push(cur);
        }
    }
    times.sort_unstable();
    let mut rem = 300;
    let mut solved = 0;
    for t in times {
        if t > rem {
            break;
        }
        rem -= t;
        solved += 1;
    }
    out.print_line(solved);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
