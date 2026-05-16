//{"name":"Fruit Feast","group":"Eolymp - Basecamp - Educational Round #9","url":"https://eolymp.com/en/compete/1fck3aasuh3ev201dcp2lm8868/problem/6","interactive":false,"timeLimit":1000,"tests":[{"input":"8 5 6\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();
    let a = input.read_size();
    let b = input.read_size();

    let mut set = BitSet::new(t + 1);
    set.set(0);
    for i in [a, b] {
        let mut cur = i;
        while cur <= t {
            set.shift_left_or(cur);
            cur *= 2;
        }
    }
    let mut nset = BitSet::new(t + 1);
    for i in set.iter() {
        nset.set(i / 2);
    }
    for i in [a, b] {
        let mut cur = i;
        while cur <= t {
            nset.shift_left_or(cur);
            cur *= 2;
        }
    }
    out.print_line(nset.iter().max());
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
