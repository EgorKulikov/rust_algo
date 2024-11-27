//{"name":"Sum of Empty the String","group":"CodeChef - START162A","url":"https://www.codechef.com/START162A/problems/CNTEMPTY","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\nABAB\n3\nBBB\n","output":"10\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SumOfEmptyTheString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut next_a = Vec::new();
    let mut next_b = Vec::new();
    let mut add = 0;
    let mut ans = 0;
    for i in 0..n {
        match s[i] {
            b'A' => {
                if let Some(next) = next_b.pop() {
                    add += i - next;
                } else {
                    add += i + 1;
                }
                next_a.push(i);
            }
            b'B' => {
                if let Some(next) = next_a.pop() {
                    add += i - next;
                } else {
                    add += i + 1;
                }
                next_b.push(i);
            }
            _ => unreachable!(),
        }
        ans += add;
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
