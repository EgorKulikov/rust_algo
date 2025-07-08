//{"name":"D. Make a Palindrome","group":"Codeforces - EPIC Institute of Technology Round Summer 2025 (Codeforces Round 1036, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2124/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5 3\n5 4 3 4 5\n4 1\n1 1 2 1\n6 6\n2 3 4 5 3 2\n5 4\n5 2 4 3 1\n8 5\n4 7 1 2 3 1 3 4\n5 4\n1 2 1 2 2\n3 3\n1 2 2\n4 4\n2 1 2 2\n","output":"YES\nYES\nYES\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Ordering;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    let b = a.clone().sorted();
    let th = b[k - 1];
    let mut c = a.into_iter().filter(|&x| x <= th).collect::<VecDeque<_>>();
    let mut total = 0;
    while c.len() > 1 {
        let x = c[0];
        let y = c[c.len() - 1];
        let max = x.max(y);
        if x != y && max < th {
            out.print_line(false);
            return;
        }
        match x.cmp(&y) {
            Ordering::Less => {
                c.pop_back();
            }
            Ordering::Equal => {
                c.pop_front();
                c.pop_back();
                total += 2;
            }
            Ordering::Greater => {
                c.pop_front();
            }
        }
    }
    total += c.len();
    out.print_line(total >= k - 1);
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
