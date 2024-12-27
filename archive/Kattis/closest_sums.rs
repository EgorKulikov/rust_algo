//{"name":"Closest Sums","group":"Kattis","url":"https://open.kattis.com/problems/closestsums","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n12\n17\n33\n34\n3\n1\n51\n30\n3\n1\n2\n3\n3\n1\n2\n3\n3\n1\n2\n3\n3\n4\n5\n6\n","output":"Case 1:\nClosest sum to 1 is 15.\nClosest sum to 51 is 51.\nClosest sum to 30 is 29.\nCase 2:\nClosest sum to 1 is 3.\nClosest sum to 2 is 3.\nClosest sum to 3 is 3.\nCase 3:\nClosest sum to 4 is 4.\nClosest sum to 5 is 5.\nClosest sum to 6 is 5.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ClosestSums"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    if input.is_empty() {
        return;
    }
    let n = input.read_size();
    let num = input.read_int_vec(n);
    let m = input.read_size();

    output!(out, "Case {}:", test_case);
    for _ in 0..m {
        let x = input.read_int();
        let mut closest = None;
        for i in 0..n {
            for j in 0..i {
                let delta = (num[i] + num[j] - x).abs();
                closest.minim((delta, num[i] + num[j]));
            }
        }
        output!(out, "Closest sum to {} is {}.", x, closest.unwrap().1);
    }
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
