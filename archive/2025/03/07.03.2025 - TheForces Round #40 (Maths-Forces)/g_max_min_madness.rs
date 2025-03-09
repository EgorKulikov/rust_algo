//{"name":"G. Max-Min Madness","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n5\n2\n0 2\n","output":"5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;

type Mod = ModIntF;
type PreCalc = Combinations<Mod>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted().reversed();

    let mut ans = Mod::zero();
    for i in 0..n {
        if a[i] == 0 {
            break;
        }
        ans += data.c(n - i + a[i] - 1, a[i] - 1);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Combinations::new(2_000_000);

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
