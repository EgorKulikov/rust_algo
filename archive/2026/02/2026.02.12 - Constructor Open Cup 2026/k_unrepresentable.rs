//{"name":"K. Unrepresentable","group":"Codeforces - Constructor Open Cup 2026","url":"https://constructor2026.contest.codeforces.com/group/XdjJUfzFUt/contest/670933/problem/K","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n2 3\n2 1\n4 9\n","output":"2\n1 2\n2 1 1\n2\n1 3\n1 2\n3\n4 12 3 1 2\n4 6 8 4 12\n3 3 2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let m = input.read_size();

    let ones = m.count_ones() as usize;
    let mut f = ones;
    if ones < k || ones >= 3 {
        f += 1;
    }
    out.print_line(f);
    for i in 0..k {
        if m.is_set(i) {
            let mut ans = Vec::new();
            ans.push(k - 1);
            for j in 0..k {
                if i != j {
                    ans.push(1 << j);
                }
            }
            out.print_line(ans);
        }
    }
    if ones < k {
        let mut ans = vec![k - 1];
        let mut first_out = true;
        let mut first_in = true;
        let mut last = 0;
        for i in 0..k {
            if m.is_set(i) {
                if first_in {
                    first_in = false;
                    last += 1 << i;
                } else {
                    ans.push(1 << i);
                }
            } else {
                if first_out {
                    first_out = false;
                    last += 1 << i;
                } else {
                    ans.push(1 << i);
                }
            }
        }
        ans.push(last);
        out.print_line(ans);
    } else if ones >= 3 {
        let mut ans = vec![k - 1, 3, 5];
        for i in 3..k {
            ans.push(1 << i);
        }
        out.print_line(ans);
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
