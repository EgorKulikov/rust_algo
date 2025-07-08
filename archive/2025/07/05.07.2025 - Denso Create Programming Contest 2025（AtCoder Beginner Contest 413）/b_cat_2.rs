//{"name":"B - cat 2","group":"AtCoder - Denso Create Programming Contest 2025（AtCoder Beginner Contest 413）","url":"https://atcoder.jp/contests/abc413/tasks/abc413_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nat\natco\ncoder\nder\n","output":"11\n"},{"input":"5\na\naa\naaa\naaaa\naaaaa\n","output":"7\n"},{"input":"10\narmiearggc\nukupaunpiy\ncogzmjmiob\nrtwbvmtruq\nqapfzsitbl\nvhkihnipny\nybonzypnsn\nesxvgoudra\nusngxmaqpt\nyfseonwhgp\n","output":"90\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::concat::StrConcat;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    let mut all = FxHashSet::default();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                all.insert(s[i].str_concat(&s[j]));
            }
        }
    }
    out.print_line(all.len());
}

pub static TEST_TYPE: TestType = TestType::Single;
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
