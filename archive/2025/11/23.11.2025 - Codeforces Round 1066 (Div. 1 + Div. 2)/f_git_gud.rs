//{"name":"F. Git Gud","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n","output":"4\n1 4\n3 1\n2 1\n3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::numbers::number_ext::Power;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n == 4 {
        out.print_line(4);
        out.print_line((1, 4));
        out.print_line((3, 1));
        out.print_line((2, 1));
        out.print_line((3, 1));
        return;
    }
    let mut cost = 0;
    let mut last = n;
    let mut ans = Vec::new();
    let step = 100;
    let num_steps = 3;
    for i in 0..num_steps {
        let i = step.power(i);
        for j in 1..step {
            for k in (0..n.upper_div(step * i)).rev() {
                let y = k * step * i + j * i;
                if y >= n {
                    continue;
                }
                let l = i.min(n - y);
                if last >= y {
                    cost += l;
                } else {
                    cost += 1000 + l;
                }
                last = y;
                ans.push((y, l));
            }
        }
    }
    let l = step.power(num_steps);
    for i in 1..n.upper_div(l) {
        let y = i * l;
        if y >= n {
            continue;
        }
        let l = l.min(n - y);
        if last >= y {
            cost += l;
        } else {
            cost += 1000 + l;
        }
        last = y;
        ans.push((y, l));
    }
    eprintln!("Cost: {}", cost);
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
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
