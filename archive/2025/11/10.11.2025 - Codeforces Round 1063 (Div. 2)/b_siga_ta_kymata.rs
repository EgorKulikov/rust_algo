//{"name":"B. Siga ta Kymata","group":"Codeforces - Codeforces Round 1063 (Div. 2)","url":"https://codeforces.com/contest/2163/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n3\n1 2 3\n010\n5\n3 4 2 1 5\n11111\n6\n1 3 2 4 6 5\n001100\n6\n6 2 3 4 5 1\n110110\n5\n2 1 4 3 5\n00000\n5\n2 5 3 1 4\n00100\n","output":"1\n1 3\n-1\n2\n1 5\n2 6\n-1\n0\n1\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();
    let x = input.read_str();

    let q = p.inv();
    if x[0] == b'1' || x[n - 1] == b'1' || x[q[0]] == b'1' || x[q[n - 1]] == b'1' {
        out.print_line(-1);
        return;
    }
    let mut ans = Vec::new();
    for from in [0, n - 1] {
        for to in [q[0], q[n - 1]] {
            if from != to {
                ans.push((from.min(to), from.max(to)));
            }
        }
    }
    ans.push((q[0].min(q[n - 1]), q[0].max(q[n - 1])));
    ans.sort();
    ans.dedup();
    out.print_line(ans.len());
    out.print_per_line(&ans.inc());
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
