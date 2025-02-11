//{"name":"B. White Magic","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5\n4 3 2 1 0\n6\n4 3 3 2 1 0\n4\n2 0 1 2\n1\n777\n4\n1000000000 1 7 9\n2\n0 1\n2\n1 2\n4\n0 1 0 1\n","output":"5\n5\n3\n1\n4\n2\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut b = Vec::new();
    let mut added_zero = false;
    for i in a {
        if i == 0 {
            if !added_zero {
                b.push(0);
                added_zero = true;
            }
        } else {
            b.push(i);
        }
    }
    if !added_zero {
        out.print_line(n);
        return;
    }
    let min = Vec::with_gen_prefix(
        b.len(),
        |i, v| if i == 0 { b[0] } else { b[i].min(v[i - 1]) },
    );
    let mut set = FxHashSet::default();
    let mut mex = 0;
    for i in (1..b.len()).rev() {
        if b[i] == mex {
            mex += 1;
            while set.contains(&mex) {
                mex += 1;
            }
        } else {
            set.insert(b[i]);
        }
        if mex > min[i - 1] {
            out.print_line(b.len() - 1);
            return;
        }
    }
    out.print_line(b.len());
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
