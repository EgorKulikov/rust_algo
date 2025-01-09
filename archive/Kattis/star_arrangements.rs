//{"name":"Star Arrangements","group":"Kattis","url":"https://open.kattis.com/problems/stararrangements","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n","output":"3:\n2,1\n"},{"input":"50\n","output":"50:\n2,1\n2,2\n3,2\n5,4\n5,5\n6,5\n10,10\n13,12\n17,16\n25,25\n"},{"input":"51\n","output":"51:\n2,1\n3,3\n9,8\n17,17\n26,25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"StarArrangements"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_int();

    let mut ans = Vec::new();
    for i in 2..s {
        for j in i - 1..=i {
            let rem = s % (i + j);
            if rem == 0 || rem == i {
                ans.push((i, j));
            }
        }
    }
    output!(out, "{}:", s);
    for (a, b) in ans {
        output!(out, "{},{}", a, b);
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
