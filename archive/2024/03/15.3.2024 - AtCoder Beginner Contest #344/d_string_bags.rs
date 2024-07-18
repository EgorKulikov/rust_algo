//{"name":"D - String Bags","group":"AtCoder - \tToyota Programming Contest 2024#3（AtCoder Beginner Contest 344）","url":"https://atcoder.jp/contests/abc344/tasks/abc344_d","interactive":false,"timeLimit":2000,"tests":[{"input":"abcde\n3\n3 ab abc abcd\n4 f c cd bcde\n2 e de\n","output":"2\n"},{"input":"abcde\n3\n2 ab abc\n3 f c bcde\n1 e\n","output":"-1\n"},{"input":"aaabbbbcccc\n6\n2 aa aaa\n2 dd ddd\n2 ab aabb\n4 bbaa bbbc bbb bbcc\n2 cc bcc\n3 ccc cccc ccccc\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DStringBags"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let t = input.read_str();
    let n = input.read_size();
    let s = input.read_vec::<Vec<Str>>(n);

    let mut mem = Memoization2d::new(n + 1, t.len() + 1, |mem, step, pos| -> Option<usize> {
        if pos == t.len() {
            return Some(0);
        }
        if step == n {
            return None;
        }
        let mut ans = mem.call(step + 1, pos);
        for s in s[step].iter() {
            if t[pos..].starts_with(&s) {
                if let Some(call) = mem.call(step + 1, pos + s.len()) {
                    ans.minim(call + 1);
                }
            }
        }
        ans
    });
    out.print_line(mem.call(0, 0));
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
    //    tester::stress_test();
}
//END MAIN
