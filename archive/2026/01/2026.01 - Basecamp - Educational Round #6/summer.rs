//{"name":"Summer","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n10 1 6\n20 1 7\n21 8 1\n","output":"250 100\n"},{"input":"3\n10 2 5\n15 2 6\n25 2 5\n","output":"400 0\n"},{"input":"2\n10 5 2\n11 6 3\n","output":"0 200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let tab = input.read_vec::<(usize, usize, usize)>(n);

    let mut bonus = BitSet::new(n);
    for i in 0..n {
        let (t1, a1, _) = tab[i];
        for j in 0..i {
            let (t2, a2, _) = tab[j];
            if t1 - t2 <= 10 && a1 == a2 {
                bonus.set(i);
            }
        }
    }
    let mut score = vec![0; 2];
    for i in 0..n {
        let a = tab[i].1;
        score[(a - 1) / 4] += 100;
        if bonus[i] {
            score[(a - 1) / 4] += 50;
        }
    }
    out.print_line(score);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
