//{"name":"Додаткові тури","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136185","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 2\n1 1\n1 3\n","output":"1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let rc = input.read_size_pair_vec(k).dec();

    let mut rows = FxHashSet::default();
    let mut cols = FxHashSet::default();
    for (r, c) in rc {
        rows.insert(r);
        cols.insert(c);
    }
    type Mod = ModIntF;
    let nn = n - rows.len();
    let mm = m - cols.len();
    if nn == mm {
        out.print_line((
            nn,
            Mod::from_index(n).power(mm) + Mod::from_index(m).power(nn) - factorial::<Mod>(nn),
        ));
    } else if nn > mm {
        out.print_line((mm, Mod::from_index(n).power(mm)));
    } else {
        out.print_line((nn, Mod::from_index(m).power(nn)));
    }
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
