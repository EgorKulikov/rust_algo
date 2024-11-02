//{"name":"T515254 繁花","group":"Luogu","url":"https://www.luogu.com.cn/problem/T515254?contestId=187558","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n<<<<\n7\n<=><=<\n9\n=<<><==<\n11\n>=<<=>>>=>\n13\n=><<=<=>=><>\n","output":"10\n9\n13\n29\n25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T515254"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let c = input.read_str();

    let mut qty = 0;
    let mut last = 1;
    let mut is_less = false;
    let mut ans = 0usize;

    for c in c {
        match c {
            b'=' => {
                last += 1;
            }
            b'<' => {
                if is_less {
                    qty += last;
                } else {
                    qty = last;
                }
                last = 1;
                is_less = true;
            }
            b'>' => {
                if is_less {
                    qty = last;
                } else {
                    qty += last;
                }
                last = 1;
                is_less = false;
            }
            _ => unreachable!(),
        }
        ans += qty;
    }
    out.print_line(ans);
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
