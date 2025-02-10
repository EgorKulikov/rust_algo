//{"name":"Happy Happy Prime Prime","group":"Kattis","url":"https://open.kattis.com/problems/happyprime","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n2 7\n3 383\n4 1000\n","output":"1 1 NO\n2 7 YES\n3 383 YES\n4 1000 NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HappyHappyPrimePrime"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::digits;
use algo_lib::numbers::primes::sieve::primality_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p = input.read_size();
    let pt = primality_table(10_001);
    let mut is_happy = BitSet::new(10_001);
    let mut done = BitSet::new(10_001);
    is_happy.set(1);
    done.set(1);

    for id in 1..=p {
        input.read_size();
        let m = input.read_size();
        let mut rec = RecursiveFunction::new(|rec, m: usize| -> bool {
            if done[m] {
                is_happy[m]
            } else {
                done.set(m);
                let res = rec.call(digits(m).map(|x| x * x).sum());
                is_happy.change(m, res);
                res
            }
        });
        out.print_line((id, m, pt[m] && rec.call(m)));
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
