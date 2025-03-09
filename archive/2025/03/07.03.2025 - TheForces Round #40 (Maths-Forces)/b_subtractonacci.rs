//{"name":"B. Subtractonacci","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 2 5\n4 1 2\n1 1 1\n3 10 3\n799843967 796619138 446173607\n1000000000 1000000000 1000000000\n","output":"10\n3\n1\n6\n649554476\n1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size() % 6 + 6;
    let a1 = input.read_int();
    let a2 = input.read_int();

    type Mod = ModInt7;
    let mut cur = Mod::new_signed(a1);
    let mut last = Mod::new_signed(a1 - a2);
    let mut sum = Mod::zero();
    for _ in 1..=n {
        sum += cur;
        let next = cur - last;
        last = cur;
        cur = next;
    }
    out.print_line(sum);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
