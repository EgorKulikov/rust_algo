//{"name":"Expansion","group":"CodeChef - START223A","url":"https://www.codechef.com/START223A/problems/PEREXP","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n2 1 3 4\n3\n1 3 3\n4\n2 4 4 3\n6\n1 6 4 6 5 6\n6\n2 4 6 2 5 6\n","output":"1\n0\n4\n0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let q = a.qty_bound(n + 1);
    let mut twos = Vec::new();
    for i in 1..=n {
        if q[i] > 2 {
            out.print_line(0);
            return;
        }
        if q[i] == 2 {
            twos.push(i);
        }
    }
    if twos.is_empty() {
        out.print_line(1);
        return;
    }
    let d = twos[0].divisors();
    let mut ans = 0;
    for d in d {
        let d = d as usize;
        let mut good = true;
        for mut i in twos.copy_iter() {
            if i % d != 0 {
                good = false;
                break;
            }
            i /= d;
            while q[i] == 1 {
                if i % d != 0 {
                    good = false;
                    break;
                }
                i /= d;
            }
            if q[i] != 0 {
                good = false;
                break;
            }
        }
        if good {
            ans += 1;
        }
    }
    type Mod = ModIntF;
    out.print_line(Mod::new(2).power(twos.len()) * ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
