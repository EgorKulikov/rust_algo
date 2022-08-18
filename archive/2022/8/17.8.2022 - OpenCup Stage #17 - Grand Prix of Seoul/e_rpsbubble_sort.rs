//{"name":"E. RPS Bubble Sort","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/E/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 1\nRSPSP\n","output":"SRPPS\n"},{"input":"10 3\nRSRRRRRRSR\n","output":"SRRRRSRRRR\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERPSBubbleSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let t = input.read_usize();
    let s: Str = input.read();

    fn beats(a: u8, b: u8) -> bool {
        assert_ne!(a, b);
        a == b'S' && b == b'P' || a == b'P' && b == b'R' || a == b'R' && b == b'S'
    }

    let mut ans = Vec::with_capacity(n);
    let mut sz = 0;
    let mut tp = b'R';
    for c in s {
        if c == tp {
            sz += 1;
            if sz > t {
                ans.push(tp);
                sz -= 1;
            }
        } else if beats(c, tp) || sz == 0 {
            for _ in 0..sz {
                ans.push(tp);
            }
            sz = 1;
            tp = c;
        } else {
            ans.push(c);
        }
    }
    for _ in 0..sz {
        ans.push(tp);
    }
    out_line!(Str::from(ans));
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
