//{"name":"D. Разноцветная печать","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5\nBRBBW\n1\nB\n2\nWB\n2\nRW\n3\nBRB\n3\nRBB\n7\nWWWWWWW\n9\nRBWBWRRBW\n10\nBRBRBRBRRB\n12\nBBBRWWRRRWBR\n10\nBRBRBRBRBW\n5\nRBWBW\n","output":"YES\nNO\nNO\nNO\nYES\nYES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRaznotsvetnayaPechat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();

    let check = |f: usize, t: usize| -> bool {
        f == t
            || s.iter()
                .skip(f)
                .take(t - f)
                .filter(|&it| it == s[f])
                .count()
                != t - f
    };
    let mut start = 0;
    for i in 0..n {
        if s[i] == b'W' {
            if !check(start, i) {
                out_line!("No");
                return;
            }
            start = i + 1;
        }
    }
    out_line!(check(start, n));
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
