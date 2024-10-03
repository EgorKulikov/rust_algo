//{"name":"GCD and XOR","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/GCDXOR","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 2\n2 6 4 1\n1 3\n3\n2 2\n4 8\n","output":"2\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCDAndXOR"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_unsigned();
    let a = input.read_unsigned_vec(n);

    if a.iter().count_eq(&&k) == n {
        out.print_line(0);
        return;
    }

    let start = a.iter().take_while(|&&x| x == k).count();
    let end = n - a.iter().rev().take_while(|&&x| x == k).count();
    let a = a[start..end].to_vec();
    if a.iter().all(|&x| x == a[0]) || a.iter().all(|&x| x % k == 0) {
        out.print_line(1);
    } else {
        out.print_line(2);
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
