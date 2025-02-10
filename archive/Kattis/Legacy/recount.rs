//{"name":"Recount","group":"Kattis","url":"https://open.kattis.com/problems/recount","interactive":false,"timeLimit":2000,"tests":[{"input":"Penny Franklin\nMarti Graham\nConnie Froggatt\nJoseph Ivers\nConnie Froggatt\nPenny Franklin\nConnie Froggatt\nBruce Stanger\nConnie Froggatt\nBarbara Skinner\nBarbara Skinner\n***\n","output":"Connie Froggatt\n"},{"input":"Penny Franklin\nConnie Froggatt\nBarbara Skinner\nConnie Froggatt\nJose Antonio Gomez-Iglesias\nConnie Froggatt\nBruce Stanger\nBarbara Skinner\nBarbara Skinner\n***\n","output":"Runoff!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Recount"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut votes = DefaultHashMap::<_, usize>::new();
    loop {
        let who = input.read_line();
        if who.as_slice() == b"***" {
            break;
        }
        votes[who] += 1;
    }

    let max = *votes.values().max().unwrap();
    let count = votes.values().filter(|&&v| v == max).count();
    if count == 1 {
        out.print_line(votes.iter().max_by_key(|&(_, v)| v).unwrap().0);
    } else {
        out.print_line("Runoff!");
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
