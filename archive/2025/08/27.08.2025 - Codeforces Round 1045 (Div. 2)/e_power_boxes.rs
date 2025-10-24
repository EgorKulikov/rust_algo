//{"name":"E. Power Boxes","group":"Codeforces - Codeforces Round 1045 (Div. 2)","url":"https://codeforces.com/contest/2134/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n4\n\n2\n\n\n3\n\n3\n\n2\n\n2\n\n\n1\n","output":"\n\nthrow 2\n\nswap 3\nthrow 2\n\nthrow 1\n\n! 2 1 2 1\n\nthrow 1\n\nswap 1\nthrow 1\n\n! 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = vec![0; n];
    let mut p = vec![0; n + 2];
    let mut order = (0..n).collect::<Vec<_>>();
    let mut i = n - 1;
    loop {
        let mut at = i;
        if p[i + 1] == p[i + 2] {
            if i == 0 {
                out.print_line("swap 1");
                out.flush();
                order.swap(0, 1);
                at = 1;
            } else {
                out.print_line(("throw", i));
                out.flush();
                let x = input.read_size();
                if x == p[i + 1] + 1 {
                    ans[order[i - 1]] = 2;
                } else {
                    ans[order[i - 1]] = 1;
                }
                out.print_line(("swap", i));
                out.flush();
                order.swap(i - 1, i);
                out.print_line(("throw", i));
                out.flush();
                let x = input.read_size();
                if x == p[i + 1] + 1 {
                    ans[order[i - 1]] = 2;
                } else {
                    ans[order[i - 1]] = 1;
                }
                for j in (i - 1..=i).rev() {
                    p[j] = 1 + p[j + ans[order[j]]];
                }
                if i == 1 {
                    break;
                }
                i -= 2;
                continue;
            }
        }
        assert_ne!(p[at + 1], p[at + 2]);
        out.print_line(("throw", at + 1));
        out.flush();
        let x = input.read_size();
        if x == p[at + 1] + 1 {
            ans[i] = 1;
        } else if x == p[at + 2] + 1 {
            ans[i] = 2;
        } else {
            unreachable!();
        }
        for j in (i..=at).rev() {
            p[j] = 1 + p[j + ans[order[j]]];
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    out.print_line(('!', ans));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
