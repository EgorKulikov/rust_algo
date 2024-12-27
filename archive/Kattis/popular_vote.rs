//{"name":"Popular Vote","group":"Kattis","url":"https://open.kattis.com/problems/vote","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10\n21\n10\n3\n20\n10\n10\n3\n10\n10\n10\n4\n15\n15\n15\n45\n","output":"majority winner 2\nminority winner 1\nno winner\nminority winner 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PopularVote"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size_vec(n);

    let sum: usize = m.iter().sum();
    let max = *m.iter().max().unwrap();
    let num_maxes = m.iter().filter(|&&x| x == max).count();

    if sum < 2 * max {
        output!(
            out,
            "majority winner {}",
            m.iter().position(|&x| x == max).unwrap() + 1
        );
    } else if num_maxes == 1 {
        output!(
            out,
            "minority winner {}",
            m.iter().position(|&x| x == max).unwrap() + 1
        );
    } else {
        out.print_line("no winner");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
