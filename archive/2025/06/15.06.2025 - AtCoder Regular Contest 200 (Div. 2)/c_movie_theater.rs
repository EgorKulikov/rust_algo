//{"name":"C - Movie Theater","group":"AtCoder - AtCoder Regular Contest 200 (Div. 2)","url":"https://atcoder.jp/contests/arc200/tasks/arc200_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 5\n2 3\n4 6\n4\n1 5\n2 6\n3 7\n4 8\n6\n6 10\n2 11\n7 8\n1 9\n3 4\n5 12\n","output":"2 1 3\n1 2 3 4\n2 4 1 5 3 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n);

    let mut ans = vec![0; n];
    let mut next = 1;
    for i in 0..n {
        if ans[i] != 0 {
            continue;
        }
        let mut rec = RecursiveFunction::new(|rec, i: usize| {
            for j in i + 1..n {
                if ans[j] == 0 && lr[j].0 >= lr[i].0 && lr[j].1 <= lr[i].1 {
                    rec.call(j);
                }
            }
            ans[i] = next;
            next += 1;
        });
        rec.call(i);
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
