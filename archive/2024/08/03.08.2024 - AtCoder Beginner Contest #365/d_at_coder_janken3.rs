//{"name":"D - AtCoder Janken 3","group":"AtCoder - Toyota Programming Contest 2024#8（AtCoder Beginner Contest 365）","url":"https://atcoder.jp/contests/abc365/tasks/abc365_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6\nPRSSRS\n","output":"5\n"},{"input":"10\nSSSSSSSSSS\n","output":"5\n"},{"input":"24\nSPRPSRRRRRPPRPRPSSRSPRSS\n","output":"18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAtCoderJanken3"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut mem = Memoization2d::new(n + 1, 4, |mem, step, last| {
        if step == n {
            return 0;
        }
        let cur = match s[step] {
            b'S' => 0,
            b'P' => 1,
            b'R' => 2,
            _ => unreachable!(),
        };
        let mut ans = 0;
        for i in 0..3 {
            if i == last || (i + 2) % 3 == cur {
                continue;
            }
            ans.maxim(mem.call(step + 1, i) + if i == cur { 0 } else { 1 });
        }
        ans
    });
    out.print_line(mem.call(0, 3));
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
}
//END MAIN
