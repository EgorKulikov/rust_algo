//{"name":"Ptice","group":"Kattis","url":"https://open.kattis.com/problems/ptice","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nBAACC\n","output":"3\nBruno\n"},{"input":"9\nAAAABBBBB\n","output":"4\nAdrian\nBruno\nGoran\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Ptice"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let iters = [
        b"ABC".into_iter().cycle().take(n),
        b"BABC".into_iter().cycle().take(n),
        b"CCAABB".into_iter().cycle().take(n),
    ];
    let names = ["Adrian", "Bruno", "Goran"];
    let mut max = 0;
    let mut who = Vec::new();
    for (name, iter) in names.iter_zip(iters) {
        let score = s.iter().zip(iter).filter(|(a, b)| a == *b).count();
        if score > max {
            max = score;
            who.clear();
            who.push(name);
        } else if score == max {
            who.push(name);
        }
    }
    out.print_line(max);
    out.print_per_line(&who);
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
