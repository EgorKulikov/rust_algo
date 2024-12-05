//{"name":"J. Make Them Believe","group":"Universal Cup - The 3rd Universal Cup. Stage 19: Shenyang","url":"https://contest.ucup.ac/contest/1865/problem/9807","interactive":false,"timeLimit":1000,"tests":[{"input":"LNG 55\nWBG 65\nHLE 70\nBLG 75\nTES 48\nT1 80\nGEN 60\nFLY 50\n","output":"T1 beats BLG\n"},{"input":"LNG 55\nWBG 65\nHLE 70\nBLG 81\nTES 48\nT1 80\nGEN 60\nFLY 50\n","output":"BLG beats T1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JMakeThemBelieve"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut teams = input.read_vec::<(Str, i32)>(8);

    teams[0..4].sort_by_key(|x| -x.1);
    teams[4..8].sort_by_key(|x| -x.1);
    if teams[0].1 > teams[4].1 {
        output!(out, "{} beats {}", teams[0].0, teams[4].0);
    } else {
        output!(out, "{} beats {}", teams[4].0, teams[0].0);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
