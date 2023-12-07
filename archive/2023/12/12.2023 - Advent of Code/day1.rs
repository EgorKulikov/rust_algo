//{"name":"day1","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    while !input.is_exhausted() {
        let s = input.read_str();

        let mut first = None;
        let mut last = None;
        for c in s.iter() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c - b'0');
                }
                last = Some(c - b'0');
            }
        }
        ans1 += (first.unwrap() * 10 + last.unwrap()) as i32;

        let mut first = None;
        let mut last = None;
        for (i, c) in s.iter().enumerate() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c - b'0');
                }
                last = Some(c - b'0');
            } else {
                for c in 1..10 {
                    if s[i..].starts_with(digits[c].as_bytes()) {
                        if first.is_none() {
                            first = Some(c as u8);
                        }
                        last = Some(c as u8);
                        break;
                    }
                }
            }
        }
        ans2 += (first.unwrap() * 10 + last.unwrap()) as i32;
    }

    {
        // part 1
        out.print_line(ans1);
    }

    {
        // part 2
        out.print_line(ans2);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
