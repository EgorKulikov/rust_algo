//{"name":"Swaps in a String","group":"CodeChef - START188A","url":"https://www.codechef.com/START188A/problems/SWAPSTR","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\nABCC\n5\nBAABB\n2\nBA\n","output":"2\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let s = input.read_str();

    let mut na = 0usize;
    let mut nb = 0;
    let mut nc = 0;
    let mut mid_b = 0;
    let mut ans = 0;
    for c in s.copy_iter() {
        if c == b'A' {
            if nc != 0 {
                ans += nc.max(na) * mid_b;
                na = 0;
                nc = 0;
                mid_b = 0;
            } else {
                ans += na * nb;
            }
            nb = 0;
            na += 1;
        } else if c == b'B' {
            nb += 1;
        } else {
            // na = 0;
            if nc == 0 {
                mid_b = nb;
            }
            nb = 0;
            nc += 1;
        }
    }
    if nc == 0 {
        ans += na * nb;
    }
    ans += nc.max(na) * mid_b;
    nb = 0;
    nc = 0;
    for c in s.copy_rev() {
        if c == b'C' {
            ans += nc * nb;
            nb = 0;
            nc += 1;
        } else if c == b'B' {
            nb += 1;
        } else {
            nb = 0;
            nc = 0;
        }
    }
    // ans += nc * nb;
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
