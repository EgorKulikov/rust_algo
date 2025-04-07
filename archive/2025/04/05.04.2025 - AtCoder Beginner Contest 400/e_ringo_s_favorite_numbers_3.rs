//{"name":"E - Ringo's Favorite Numbers 3","group":"AtCoder - AtCoder Beginner Contest 400","url":"https://atcoder.jp/contests/abc400/tasks/abc400_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n404\n36\n60\n1000000000000\n123456789\n","output":"400\n36\n36\n1000000000000\n123454321\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::numbers::number_ext::Square;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let dt = divisor_table(1_000_001);
    let ddt: Vec<i32> = Vec::with_gen_prefix(1_000_001, |mut i, ddt| {
        if i <= 1 {
            return 0;
        }
        let d = dt[i];
        while i % d == 0 {
            i /= d;
        }
        ddt[i] + 1
    });
    let mut twos = Vec::new();
    for i in 1..=1_000_000 {
        if ddt[i] == 2 {
            twos.push(i);
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let a = input.read_long();
        let aa = a.lower_sqrt() as usize;
        let pos = twos.upper_bound(&aa) - 1;
        out.print_line(twos[pos].square());
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
