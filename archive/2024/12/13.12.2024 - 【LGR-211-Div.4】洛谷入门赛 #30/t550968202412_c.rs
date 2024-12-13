//{"name":"T550968 202412C 古希腊掌管罚时的神","group":"Luogu","url":"https://www.luogu.com.cn/problem/T550968?contestId=219467","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 0\n2 0\n3 0\n35 1\n91 1\n","output":"186\n"},{"input":"10\n2 1\n35 1\n40 1\n47 1\n54 1\n63 0\n70 1\n75 0\n93 1\n97 1\n","output":"478\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T550968202412C"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let submissions = input.read_size_pair_vec(n);

    let mut ans = 0;
    for (a, b) in submissions {
        if b == 0 {
            ans += 20;
        } else {
            ans += a;
        }
    }
    out.print_line(ans);
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
