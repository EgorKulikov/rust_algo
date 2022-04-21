//{"name":"C. Half Queen Cover","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n","output":"1\n1 1\n"},{"input":"2\n","output":"1\n1 1\n"},{"input":"3\n","output":"2\n1 1\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHalfQueenCover"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    if n == 1 {
        out_line!(1);
        out_line!(1, 1);
        return;
    }

    let x = (n + 1) / 3;
    let mut ans = Vec::new();
    for i in 1..=x {
        ans.push((i, x + 1 - i));
    }
    for i in 1..x {
        ans.push((x + i, 2 * x - i));
    }
    for i in 2 * x..=n - x {
        ans.push((i, i));
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
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
