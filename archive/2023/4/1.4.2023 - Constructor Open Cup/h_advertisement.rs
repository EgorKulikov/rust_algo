//{"name":"H. Advertisement","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nmm\nabcca\nconstructor\ncontest\ncodeforces\n","output":"26\n7\n93\n59\n67\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HAdvertisement"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut mem = Memoization2d::new(s.len() + 1, 64, |f, pos, mask| {
        if pos == s.len() {
            return 0;
        }
        let mut res = f.call(pos + 1, (mask << 1) & 63);
        if (mask & 3) != 3 && mask.count_ones() < 3 {
            res.maxim(f.call(pos + 1, (mask << 1) & 63 | 1) + (s[pos] - b'a' + 1).into_usize());
        }
        res
    });
    out_line!(mem.call(0, 0));
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
