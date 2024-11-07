//{"name":"B. K Palindrome","group":"Codeforces - TheForces Round #37 (Brute-Forces1)","url":"https://codeforces.com/gym/105491/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1\n2 1\n2 2\n3 2\n3 26\n","output":"z\nkk\nkk\n-1\nwww\nwyw\nwww\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKPalindrome"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let half = (n + 1) / 2;
    if k > half * half {
        out.print_line(-1);
        return;
    }
    let ans = Arr2d::generate(n, n, |mut i, mut j| {
        if i >= half {
            i = n - i - 1;
        }
        if j >= half {
            j = n - j - 1;
        }
        let id = (i * half + j) % k;
        b'a' + id as u8
    });
    out.print_table(&ans);
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
