//{"name":"A2. Encode and Decode (Hard Version)","group":"Codeforces - Testing Round 20 (Unrated, Communication Problems)","url":"https://codeforces.com/contest/2168/problem/A2","interactive":false,"timeLimit":2000,"tests":[{"input":"first\n5\n100 200 300 400 500\n","output":"skibidi\n"},{"input":"second\nskibidi\n","output":"5\n100 200 300 400 500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_str();
    if t.as_slice() == b"first" {
        let n = input.read_size();
        let a = input.read_size_vec(n).dec();

        let mut s = Str::new();
        for i in 0..n {
            let mut cur = a[i];
            for _ in 0..9 {
                let d = cur % 10;
                s.push(b'a' + d as u8);
                cur /= 10;
            }
        }
        out.print_line(s);
    } else {
        let s = input.read_str();
        let a = Vec::with_gen(s.len() / 9, |i| {
            let mut res = 0;
            for j in (0..9).rev() {
                res = res * 10 + (s[i * 9 + j] - b'a') as usize;
            }
            res
        });
        out.print_line(a.len());
        out.print_line(a.inc());
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::RunTwice;

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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
