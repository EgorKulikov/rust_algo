//{"name":"day8","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day8"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::lcm;
use algo_lib::scan;
use algo_lib::string::str::{Str, StrReader};
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let path = input.read_str();
    let mut map = HashMap::new();
    input.read_line();

    while !input.is_exhausted() {
        scan!(input, "@ = (@, @)", from: Str<'static>, to1: Str<'static>, to2: Str<'static>);
        map.insert(from, (to1, to2));
    }

    {
        // part 1
        let mut ans = 0;
        let mut at = &Str::from(b"AAA");
        while at != &b"ZZZ".into() {
            let (to1, to2) = &map[&at];
            if path[ans % path.len()] == b'L' {
                at = to1;
            } else {
                at = to2;
            }
            ans += 1;
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut ans = 1;
        let at = map.keys().filter(|s| s[2] == b'A').collect_vec();
        for mut at in at {
            let mut cur = 0;
            while at[2] != b'Z' {
                let (to1, to2) = &map[&at];
                if path[cur % path.len()] == b'L' {
                    at = to1;
                } else {
                    at = to2;
                }
                cur += 1;
            }
            ans = lcm(ans, cur);
        }
        out.print_line(ans);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
