//{"name":"B. Создайте три региона","group":"Codeforces - Educational Codeforces Round 168 (Rated for Div. 2)","url":"https://codeforces.com/contest/1997/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n8\n.......x\n.x.xx...\n2\n..\n..\n3\nxxx\nxxx\n9\n..x.x.x.x\nx.......x\n","output":"1\n0\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSozdaiteTriRegiona"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(2);

    let mut ans = 0;
    for i in 1..n - 1 {
        for j in 0..2 {
            if s[j][i] == b'.'
                && s[j][i - 1] == b'.'
                && s[j][i + 1] == b'.'
                && s[1 - j][i] == b'.'
                && s[1 - j][i - 1] == b'x'
                && s[1 - j][i + 1] == b'x'
            {
                ans += 1;
            }
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
    let test_type = TestType::MultiNumber;
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
