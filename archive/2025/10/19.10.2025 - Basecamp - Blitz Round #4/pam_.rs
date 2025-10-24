//{"name":"PAM!","group":"Eolymp - Basecamp - Blitz Round #4","url":"https://eolymp.com/en/compete/7mk1e6onrl4pb69590dkne46j4/problem/3","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n1 1 1\n5 7\n2 5 5 3 4\n10 10\n3 7 3 7 7 3 2 8 8 8\n","output":"1 1 1\n3 4 5 2 5\n3 7 3 7 3 7 8 8 2 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n);

    let mut q = DefaultTreeMap::new(0);
    for i in a {
        q[i] += 1;
    }
    let mut ans = Vec::new();
    for (&c, &v) in q.iter() {
        if c * 2 < k {
            let v2 = q[k - c];
            if v >= v2 {
                for _ in 0..v2 {
                    ans.push(c);
                    ans.push(k - c);
                }
                for _ in 0..(v - v2) {
                    ans.push(c);
                }
            } else {
                for _ in 0..v {
                    ans.push(k - c);
                    ans.push(c);
                }
                for _ in 0..(v2 - v) {
                    ans.push(k - c);
                }
            }
        } else if q[k - c] == 0 || c * 2 == k {
            for _ in 0..v {
                ans.push(c);
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
