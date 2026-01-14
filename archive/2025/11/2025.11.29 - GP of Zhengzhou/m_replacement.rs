//{"name":"M. Replacement","group":"Universal Cup - GP of Zhengzhou","url":"https://contest.ucup.ac/contest/2661/problem/15313","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n010\n0110\n1000\n10101\n01000\n","output":"0\n10\n10\n100\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    fn trim(s: &mut Str) {
        for i in 0..s.len() {
            if s[i] == b'1' {
                *s = Str::from(&s[i..]);
                return;
            }
        }
        *s = Str::from(b"0");
    }

    let mut ans = Str::from(&s[..s.len() - 2]);
    if s[Back(0)] == b'1' {
        ans[Back(0)] ^= 1;
    }
    trim(&mut ans);
    let mut cand = Str::from(&s[2..]);
    let mut len = 1;
    for i in 2..s.len() - 2 {
        if s[i] == b'1' {
            break;
        }
        len += 1;
    }
    if s[0] == b'1' {
        let mut found = false;
        for j in (0..len).rev() {
            if cand[Back(j)] == b'0' {
                cand[Back(j)] ^= 1;
                found = true;
                if s[1] == b'1' && j > 0 {
                    cand[Back(j - 1)] ^= 1;
                }
                break;
            }
        }
        if !found {
            cand[Back(0)] ^= 1;
        }
    } else if s[1] == b'1' {
        for j in (0..len - 1).rev() {
            if cand[Back(j)] == b'0' {
                cand[Back(j)] ^= 1;
                break;
            }
        }
    }
    trim(&mut cand);
    if cand.len() > ans.len() || cand.len() == ans.len() && cand > ans {
        ans = cand;
    }
    out.print_line(ans);
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
