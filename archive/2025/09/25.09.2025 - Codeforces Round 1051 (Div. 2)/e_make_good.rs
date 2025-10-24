//{"name":"E. Make Good","group":"Codeforces - Codeforces Round 1051 (Div. 2)","url":"https://codeforces.com/contest/2143/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n()()\n6\n((())(\n10\n))(())())(\n8\n))))))))\n1\n(\n","output":"()()\n-1\n-1\n(())(())\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut s = input.read_str();

    if n % 2 == 1 {
        out.print_line(-1);
        return;
    }
    let open = s.copy_count(b'(');
    if open % 2 != n / 2 % 2 {
        out.print_line(-1);
        return;
    }
    if s[0] != b'(' {
        let mut good = false;
        for i in 1..n {
            if s[i] == s[i - 1] {
                if s[i] == b')' {
                    s[i] = b'(';
                    s[i - 1] = b'(';
                }
                if s[0] == b')' {
                    let mut t = Str::new();
                    t.push(b'(');
                    t.push(b'(');
                    t.extend_from_slice(&s[0..i - 1]);
                    t.extend_from_slice(&s[i + 1..n]);
                    s = t;
                }
                good = true;
                break;
            }
        }
        if !good {
            out.print_line(-1);
            return;
        }
    }
    if s[n - 1] != b')' {
        let mut good = false;
        for i in (1..n).rev() {
            if s[i] == s[i - 1] {
                if s[i] == b'(' {
                    s[i] = b')';
                    s[i - 1] = b')';
                }
                if s[n - 1] == b'(' {
                    let mut t = Str::new();
                    t.extend_from_slice(&s[0..i - 1]);
                    t.extend_from_slice(&s[i + 1..n]);
                    t.push(b')');
                    t.push(b')');
                    s = t;
                }
                good = true;
                break;
            }
        }
        if !good {
            out.print_line(-1);
            return;
        }
    }
    assert_eq!(s[0], b'(');
    assert_eq!(s[n - 1], b')');
    for i in 1..n {
        if s[i - 1] == b')' && s[i] == b')' {
            s[i - 1] = b'(';
            s[i] = b'(';
        }
    }
    let open = s.copy_count(b'(');
    if open < n / 2 {
        let mut need = n / 2 - open;
        for i in 1..n {
            if s[i - 1] == b')' && s[i] == b')' {
                s[i - 1] = b'(';
                s[i] = b'(';
                need -= 2;
                if need == 0 {
                    break;
                }
            }
        }
    } else if open > n / 2 {
        let mut need = open - n / 2;
        for i in (1..n).rev() {
            if s[i - 1] == b'(' && s[i] == b'(' {
                s[i - 1] = b')';
                s[i] = b')';
                need -= 2;
                if need == 0 {
                    break;
                }
            }
        }
    }
    out.print_line(s);
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
