//{"name":"C. Mr. Wow and Spells","group":"Codeforces - TheForces Round #33(Wow-Forces)","url":"https://codeforces.com/gym/105293/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 2 3\n3\n3 2 1\n3\n2 1 4\n4\n2 3 8 4\n4\n7 17 8 11\n","output":"1\n3\n3\n7\n23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMrWowAndSpells"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_long_vec(n);

    let mut ans = h[0] - 1;
    let mut forbidden_up_to = 1;
    for i in 1..n {
        let h = h[i];
        if h == forbidden_up_to + 1 {
            forbidden_up_to += 1;
            continue;
        }
        ans += (h - 1) / (forbidden_up_to + 1);
    }
    out.print_line(ans + 1);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
