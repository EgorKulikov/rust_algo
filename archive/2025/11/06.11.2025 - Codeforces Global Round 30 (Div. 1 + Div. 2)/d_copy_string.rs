//{"name":"D. Copy String","group":"Codeforces - Codeforces Global Round 30 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2164/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4 1\nabcd\naabd\n2 2\nab\nab\n5 3\nabcde\nabbcc\n9 1\negcnyeluw\neegccyelw\n10 3\nvzvylxxmsy\nvvvvvllxxx\n4 6\nacba\naaac\n5 7\nacabb\naaaca\n","output":"1\naabd\n0\n2\nabbcd\nabbcc\n-1\n3\nvvzvylxxms\nvvvzvllxxm\nvvvvvllxxx\n2\naacb\naaac\n2\naacab\naaaca\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut s = input.read_str();
    let mut t = input.read_str();

    s.reverse();
    t.reverse();
    let mut need = 0;
    let mut at = 0;
    let mut pos = vec![0; n];
    for i in 0..n {
        at.maxim(i);
        while at < n && s[at] != t[i] {
            at += 1;
        }
        if at == n {
            out.print_line(-1);
            return;
        }
        pos[i] = at;
        need.maxim(at - i);
    }
    if need > k {
        out.print_line(-1);
        return;
    }
    out.print_line(need);
    for _ in 0..need {
        for i in 0..n {
            if pos[i] > i {
                s[pos[i] - 1] = s[pos[i]];
                pos[i] -= 1;
            }
        }
        s.reverse();
        out.print_line(&s);
        s.reverse();
    }
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
