//{"name":"C - Election","group":"AtCoder - AtCoder Regular Contest 172","url":"https://atcoder.jp/contests/arc172/tasks/arc172_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nAABB\n","output":"3\n"},{"input":"4\nAAAA\n","output":"1\n"},{"input":"10\nBBBAAABBAA\n","output":"5\n"},{"input":"172\nAABAAAAAABBABAABBBBAABBAAABBABBABABABBAAABAAABAABAABBBBABBBABBABBBBBBBBAAABAAABAAABABBBAABAAAABABBABBABBBBBABAABAABBBABABBAAAABAABABBBABAAAABBBBABBBABBBABAABBBAAAABAAABAAAB\n","output":"24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CElection"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let _n = input.read_size();
    let c = input.read_str();

    let mut ans = 1;
    let mut a = 0;
    let mut b = 0;
    for cc in c.iter().skip(1) {
        if cc != c[0] {
            if a == b || a + 1 == b || a == b + 1 {
                ans += 1;
            }
        }
        if cc == b'A' {
            a += 1;
        } else {
            b += 1;
        }
    }
    out.print_line(ans);
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
