//{"name":"D. Construct it !","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4 6\n","output":"7\n1 4 4 4 5 5 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let k = input.read_long();

    let mut at = 1_000_000_000;
    let mut ans = Vec::new();
    for i in 0..60 {
        if k.is_set(i) {
            at -= 2 * l;
            let mut cur = at;
            for _ in 0..i {
                ans.push(cur + 1);
                ans.push(cur);
                cur += 2;
            }
            for _ in i..l {
                ans.push(cur);
                cur += 1;
            }
        }
    }
    out.print_line(ans.len());
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
