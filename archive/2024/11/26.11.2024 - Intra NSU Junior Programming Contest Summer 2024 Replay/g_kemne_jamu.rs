//{"name":"G - Kemne Jamu?","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6458","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nSAC1-NAC2\nNAC5-SAC5\nSAC4-SAC4\nSAC4-NAC4\nSAC10-SAC2\n","output":"Case 1: 1\nCase 2: 0\nCase 3: 0\nCase 4: 2\nCase 5: 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GKemneJamu"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    scan!(input, "@AC@-@AC@\n", l_side: u8, l_floor: i32, r_side: u8, r_floor: i32);

    out.print(format!("Case {}: ", test_case));
    if l_side == r_side {
        out.print_line((l_floor - r_floor).abs());
    } else {
        let mut ans = None;
        for i in [1, 2, 3, 5] {
            ans.minim((l_floor - i).abs() + (r_floor - i).abs());
        }
        out.print_line(ans);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
