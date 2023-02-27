//{"name":"A. Монстры (простая версия)","group":"Codeforces - VK Cup 2022 - Финальный раунд (Engine)","url":"https://codeforces.com/contest/1784/problem/A","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3\n3 1 2\n6\n4 1 5 4 1 1\n","output":"0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMonstriProstayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    a.sort();
    let mut last = 0;
    let mut ans = 0;
    for i in a {
        last += 1;
        if last <= i {
            ans += i - last;
        } else {
            last = i;
        }
    }
    out_line!(ans);
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
