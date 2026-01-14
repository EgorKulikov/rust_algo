//{"name":"C - 2x2 Placing","group":"AtCoder - AtCoder Beginner Contest 436","url":"https://atcoder.jp/contests/abc436/tasks/abc436_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 1\n2 2\n2 3\n","output":"2\n"},{"input":"1000000000 4\n1 1\n1 101\n101 1\n101 101\n","output":"4\n"},{"input":"8 10\n6 5\n7 3\n6 7\n3 4\n4 2\n3 7\n1 3\n7 4\n6 1\n6 1\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let pos = input.read_size_pair_vec(m);

    let mut set = FxHashSet::default();
    let mut ans = 0;
    for (r, c) in pos {
        let mut good = true;
        for x in r..=r + 1 {
            for y in c..=c + 1 {
                if set.contains(&(x, y)) {
                    good = false;
                }
            }
        }
        if good {
            ans += 1;
            for x in r..=r + 1 {
                for y in c..=c + 1 {
                    set.insert((x, y));
                }
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
