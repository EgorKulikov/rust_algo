//{"name":"C. MEX Cycle","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 1 3\n4 2 4\n6 3 5\n7 3 6\n3 2 3\n5 1 5\n6 2 5\n","output":"0 2 1 0 1\n1 2 1 0\n1 2 0 1 2 0\n0 1 2 0 1 0 1\n2 0 1\n1 0 2 1 0\n0 1 2 0 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMEXCycle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut x = input.read_size() - 1;
    let mut y = input.read_size() - 1;

    let mut ans = vec![0; n];
    for i in x..x + n {
        ans[i % n] = (i - x) % 2;
    }
    if ans[(x + n - 1) % n] == 0 {
        ans[(x + n - 1) % n] = 2;
    }
    if ans[y] == 0 {
        ans[y] = 2;
    }
    if (y + 3) % n == x {
        swap(&mut x, &mut y);
        for i in x..x + n {
            ans[i % n] = (i - x) % 2;
        }
        if ans[(x + n - 1) % n] == 0 {
            ans[(x + n - 1) % n] = 2;
        }
        if ans[y] == 0 {
            ans[y] = 2;
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
