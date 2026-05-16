//{"name":"Roads renewal","group":"Eolymp - Basecamp - Educational Round #9","url":"https://eolymp.com/en/compete/1fck3aasuh3ev201dcp2lm8868/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"4 3\n2 1\n1 3\n4 3\n","output":"1\n"},{"input":"5 5\n2 1\n1 3\n2 3\n2 5\n4 3\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let xy = input.read_size_pair_vec(m).dec();

    let mut dsu = DSU::new(n);
    let mut cycles = Vec::new();
    for (x, y) in xy {
        if !dsu.union(x, y) {
            cycles.push((x, y));
        }
    }
    let mut in_cycle = BitSet::new(n);
    for (x, _) in cycles {
        in_cycle.set(dsu.find(x));
    }
    out.print_line(dsu.set_count() - in_cycle.count_ones());
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
