//{"name":"A Vicious Pikeman (Hard)","group":"Kattis","url":"https://open.kattis.com/problems/pikemanhard","interactive":false,"timeLimit":1000,"tests":[{"input":"1 3\n2 2 2 1\n","output":"1 1\n"},{"input":"2 10\n2 2 2 2\n","output":"2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let t0 = input.read_long();

    let mut p = vec![t0];
    let mut was = FxHashMap::default();
    was.insert(t0, 0);
    let mut prefix = 0;
    for i in 0.. {
        let next = (a * p[i] + b) % c + 1;
        if let Some(p) = was.insert(next, i + 1) {
            prefix = p;
            break;
        }
        p.push(next);
    }
    let mut problems = Vec::new();
    for i in p.indices() {
        if i >= n {
            break;
        }
        if i < prefix {
            problems.push((p[i], 1));
        } else {
            let rem = n - i - 1;
            problems.push((p[i], rem / (p.len() - prefix) + 1));
        }
    }
    problems.sort();
    type Mod = ModInt7;
    let mut solved = 0;
    let mut cur = 0;
    let mut penalty = Mod::zero();
    for (time, qty) in problems {
        let can_solve = ((t - cur) / time).min(qty as i64);
        solved += can_solve;
        cur += time * can_solve;
        penalty += (Mod::from(cur) + cur - time * (can_solve - 1)) * can_solve / 2;
    }
    out.print_line((solved, penalty));
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
