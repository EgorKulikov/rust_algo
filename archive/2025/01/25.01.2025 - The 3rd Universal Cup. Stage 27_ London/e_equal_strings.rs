//{"name":"E. Equal Strings","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8615","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10110000010010010101011100011000000011001011000101\n10101101000111001111100000010010000100101011100101\n10011110110010011111100111011000010100011101111011\n10101101000111001111100000010010000100101011100101\n","output":"\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let q = n.min(20);
    let mut res = vec![Vec::new(); n];
    for i in 0..q {
        for j in 0..n {
            if i != j {
                out.print_line((i + 1, j + 1));
                out.flush();
                let d = input.read_int();
                if d == 0 {
                    return;
                }
                res[j].push(d);
            } else {
                res[j].push(0);
            }
        }
    }
    let by_id = by_index(&res);
    for v in by_id.into_values() {
        if v.len() > 1 {
            for i in v.indices() {
                for j in 0..i {
                    out.print_line((v[i] + 1, v[j] + 1));
                    out.flush();
                    if input.read_int() == 0 {
                        return;
                    }
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
