//{"name":"D2. Little String (Hard Version)","group":"Codeforces - Codeforces Round 1075 (Div. 2)","url":"https://codeforces.com/contest/2189/problem/D2","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n3 3\n00?\n3 1\n???\n4 100\n1001\n3 3\n???\n6 100\n111001\n6 100\n111101\n5 8\n100?1\n4 100\n1??0\n20 253034496\n10001100011000??????\n3 4\n1?1\n","output":"-1\n-1\n4\n2\n96\n64\n12\n-1\n833286105\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut c = input.read_size();
    let s = input.read_str();

    if s[0] == b'0' || s[n - 1] == b'0' {
        out.print_line(-1);
        return;
    }
    type Mod = ModInt7;
    let mut ans = Mod::from(2);
    if c % 2 == 0 {
        c /= 2;
    }
    for i in 1..n - 1 {
        if s[i] == b'0' {
            ans *= i;
            c /= gcd(c, i);
        } else if s[i] == b'1' || i % 2 == 0 {
            ans *= 2;
            c /= gcd(c, 2);
        }
    }
    let mut rem = if c.count_ones() == 1 {
        if c == 1 {
            out.print_line(-1);
            return;
        }
        c.lowest_bit() - 1
    } else {
        n
    };
    for i in (2..n - 1).rev() {
        if s[i] == b'?' && i % 2 != 0 {
            if rem > 0 {
                rem -= 1;
                ans *= 2;
            } else {
                ans *= i;
            }
        }
    }
    out.print_line(ans);
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
