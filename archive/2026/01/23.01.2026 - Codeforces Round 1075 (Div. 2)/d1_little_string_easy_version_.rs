//{"name":"D1. Little String (Easy Version)","group":"Codeforces - Codeforces Round 1075 (Div. 2)","url":"https://codeforces.com/contest/2189/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n3 3\n001\n3 1\n111\n4 100\n1001\n6 100\n111001\n6 100\n111101\n5 8\n10001\n4 100\n1110\n21 123456789\n111000111000111000111\n3 4\n101\n","output":"-1\n-1\n4\n96\n64\n12\n-1\n336892528\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::{ModInt, ModInt7};
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_unsigned();
    let s = input.read_str();

    if s[0] == b'0' {
        out.print_line(-1);
        return;
    }
    type Mod = ModInt7;
    dynamic_value!(Modulo: u32 = c);
    type Mod2 = ModInt<Modulo>;
    let mut q = Mod::one();
    let mut q2 = Mod2::one();
    let mut has = 1;
    for i in 1..n {
        if s[i] == b'1' {
            q *= 2;
            q2 *= 2;
            for _ in 0..i - has {
                q *= has;
                q2 *= has;
                has += 1;
            }
            has += 1;
        }
    }
    if has != n || q2 == Mod2::zero() {
        out.print_line(-1);
    } else {
        out.print_line(q);
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
