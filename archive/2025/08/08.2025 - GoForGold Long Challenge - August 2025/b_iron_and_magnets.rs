//{"name":"B. Iron and Magnets","group":"Codeforces - GoForGold Long Challenge - August 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/629150/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 6 3\n1 2 1\n5\n1 2 3 4 5\n0 1 0 2 0\n4\n0 10 20 30\n5 5 5 5\n","output":"3\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int_vec(n);
    let a = input.read_int_vec(n);

    #[derive(Eq, Ord, PartialOrd, PartialEq)]
    enum Type {
        Pos,
        End(i32),
    }
    let mut events = Vec::with_capacity(2 * n);
    for i in 0..n {
        events.push((x[i] + a[i], Type::End(x[i] - a[i])));
        events.push((x[i], Type::Pos));
    }
    events.sort();
    let mut ans = 0;
    let mut last_seen = i32::MIN;
    let mut last_selected = i32::MIN;
    for (at, t) in events {
        match t {
            Type::End(start) => {
                if start > last_selected {
                    last_selected = last_seen;
                    ans += 1;
                }
            }
            Type::Pos => {
                last_seen = at;
            }
        }
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
