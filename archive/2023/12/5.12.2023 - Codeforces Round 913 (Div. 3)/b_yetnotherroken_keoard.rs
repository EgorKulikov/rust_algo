//{"name":"B. YetnotherrokenKeoard","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"12\nARaBbbitBaby\nYetAnotherBrokenKeyboard\nBubble\nImprobable\nabbreviable\nBbBB\nBusyasaBeeinaBedofBloomingBlossoms\nCoDEBARbIES\ncodeforces\nbobebobbes\nb\nTheBBlackbboard\n","output":"ity\nYetnotherrokenKeoard\nle\nImprle\nrevile\n\nusyasaeeinaedofloominglossoms\nCDARIES\ncodeforces\nes\n\nhelaoard\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYetnotherrokenKeoard"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let mut ans = Str::new();
    let mut take = Vec::new();
    let mut small = Vec::new();
    let mut capital = Vec::new();
    for c in s {
        match c {
            b'b' => {
                if let Some(last) = small.pop() {
                    take[last] = false;
                }
            }
            b'B' => {
                if let Some(last) = capital.pop() {
                    take[last] = false;
                }
            }
            c if c.is_ascii_lowercase() => {
                take.push(true);
                small.push(take.len() - 1);
                ans.push(c);
            }
            c if c.is_ascii_uppercase() => {
                take.push(true);
                capital.push(take.len() - 1);
                ans.push(c);
            }
            _ => unreachable!(),
        }
    }
    let ans = ans
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| if take[i] { Some(c) } else { None })
        .collect::<Str>();
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
