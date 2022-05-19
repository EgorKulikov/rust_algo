//{"name":"J. Rounding Master","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/J/","interactive":false,"timeLimit":1000,"tests":[{"input":"18 4\n","output":"2.125000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JRoundingMaster"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_float();
    let m = input.read_usize();

    if n == 1. {
        out_line!("0.5");
        return;
    }

    let mut left = 1.5;
    let mut right = n;
    for _ in 0..100 {
        let mid = (left + right) / 2.;
        let mut cur = 1.;
        for _ in 0..m {
            if cur >= n {
                break;
            }
            cur *= mid;
            cur = cur.round();
        }
        if cur >= n {
            right = mid;
        } else {
            left = mid;
        }
    }
    out_line!((left + right) / 2.);
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
