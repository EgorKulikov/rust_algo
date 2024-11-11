//{"name":"C - ∠","group":"LightOJ","url":"https://lightoj.com/contest/20241104/arena/problem/6347","interactive":false,"timeLimit":1500,"tests":[{"input":"5 5\n1 2 3\n3 2 1\nreflex 1 5 3\n4 5 3\n5 3 2\n","output":"108.0000\n108.0000\n288.0000\n36.0000\n72.0000\n"},{"input":"7 5\nreflex 1 5 4\n3 2 1\n7 2 6\n7 6 2\nreflex 6 5 3\n","output":"282.8571\n128.5714\n25.7143\n51.4286\n257.1429\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::IntoReal;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    out.set_precision(Some(4));
    let single = 180. / n.into_real();
    for _ in 0..m {
        let reflex = if input.peek() == Some(b'r') {
            input.read_str();
            true
        } else {
            false
        };
        let mut a = input.read_size();
        let b = input.read_size();
        let mut c = input.read_size();
        if a > c {
            std::mem::swap(&mut a, &mut c);
        }
        let ans = if a < b && b < c {
            single * (n - (c - a)).into_real()
        } else {
            single * (c - a).into_real()
        };
        let ans = if reflex { 360. - ans } else { ans };
        out.print_line(ans);
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
