//{"name":"T539825 202411F Rise","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539825?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6\nwater 2 4\nwater 3 5\nrise 1 3 2\nwater 1 5\nrise 1 5 1\nrise 1 5 1\n","output":"1\n5\n0\n"},{"input":"2 3\nrise 1 2 1\nwater 1 1\nrise 1 2 1\n","output":"0\n1\n"},{"input":"1 4\nwater 1 1\nwater 1 1\nrise 1 1 3\nrise 1 1 2\n","output":"0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539825202411FRise"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    let mut h = vec![0; n];
    for _ in 0..q {
        match input.read_str().as_slice() {
            b"water" => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                for i in l..r {
                    h[i] += 1;
                }
            }
            b"rise" => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let k = input.read_size();

                let mut c = 0;
                for i in l..r {
                    if h[i] >= k {
                        h[i] = 0;
                        c += 1;
                    }
                }
                out.print_line(c);
            }
            _ => unreachable!(),
        }
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
