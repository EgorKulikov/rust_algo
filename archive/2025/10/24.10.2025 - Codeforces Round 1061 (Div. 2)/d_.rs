//{"name":"D. Найдите последнее число","group":"Codeforces - Codeforces Round 1061 (Div. 2)","url":"https://codeforces.com/contest/2156/problem/D","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n2\n\n0\n\n1\n\n3\n\n1\n\n0\n\n1\n","output":"\n\n? 1 1\n\n? 1 2\n\n! 1\n\n? 1 3\n\n? 1 2\n\n? 2 3\n\n! 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut positions = (1..n).collect::<Vec<_>>();
    let mut rem = (1..=n).collect::<Vec<_>>();
    for i in 0.. {
        let mut has = Vec::new();
        let mut has_not = Vec::new();
        for j in positions {
            output!(out, "? {} {}", j, 1 << i);
            out.flush();
            if input.read_int() == 1 {
                has.push(j);
            } else {
                has_not.push(j);
            }
        }
        let mut rem_has = Vec::new();
        let mut rem_not = Vec::new();
        for j in rem {
            if j.is_set(i) {
                rem_has.push(j);
            } else {
                rem_not.push(j);
            }
        }
        if has.len() == rem_has.len() {
            positions = has_not;
            rem = rem_not;
        } else {
            positions = has;
            rem = rem_has;
        }
        if positions.is_empty() {
            break;
        }
    }
    output!(out, "! {}", rem[0]);
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
