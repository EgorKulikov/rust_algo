//{"name":"The Dragon of Loowater","group":"Kattis","url":"https://open.kattis.com/problems/loowater","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n5\n4\n7\n8\n4\n2 1\n5\n5\n10\n0 0\n","output":"11\nLoowater is doomed!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_int_vec(n).sorted();
    let h = input.read_int_vec(m).sorted();

    if n == 0 {
        return;
    }
    let all = d
        .iter_map(|x| (x, false))
        .chain(h.iter_map(|x| (x, true)))
        .collect::<Vec<_>>()
        .sorted();
    let mut ans = 0;
    let mut heads = 0;
    for (x, is_knight) in all {
        if !is_knight {
            heads += 1;
        } else {
            if heads > 0 {
                heads -= 1;
                ans += x;
            }
        }
    }
    if heads == 0 {
        output!(out, "{}", ans);
    } else {
        output!(out, "Loowater is doomed!");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
