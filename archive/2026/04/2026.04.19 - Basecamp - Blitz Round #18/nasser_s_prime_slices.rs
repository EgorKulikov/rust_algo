//{"name":"Nasser's Prime Slices","group":"Eolymp - Basecamp - Blitz Round #18","url":"https://eolymp.com/en/compete/r3hr8hsui169n3libfqfgmb0no/problem/4","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 2\n2 3 5 7 11 13\n4 2\n2 3 4 5\n3 1\n3 4 2\n6 2\n2 2 1 1 2 2\n5 5\n5 5 5 5 5\n","output":"1\n0\n2\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::primes::sieve::primality_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let pt = primality_table(10_000_001);
    let t = input.read_size();

    for _ in 0..t {
        let n = input.read_size();
        let k = input.read_size();
        let a = input.read_size_vec(n);

        type Mod = ModInt7;
        let mut len = 0;
        let mut qty = 0;
        let mut ans = Mod::new(1);
        for i in a {
            if pt[i] {
                if qty % k == 0 && qty != 0 {
                    ans *= len + 1;
                }
                qty += 1;
                len = 0;
            } else {
                len += 1;
            }
        }
        if qty % k != 0 || qty == 0 {
            out.print_line(0);
        } else {
            out.print_line(ans);
        }
    }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
