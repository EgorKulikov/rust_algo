//{"name":"A. Разделяй и властвуй","group":"Codeforces - Codeforces Round #838 (Div. 2)","url":"https://codeforces.com/contest/1762/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 1 1 1\n2\n7 4\n3\n1 2 4\n1\n15\n","output":"0\n2\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARazdelyaiIVlastvui"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let sum = a.iter().sum::<usize>();
    if sum % 2 == 0 {
        out_line!(0);
        return;
    }
    let mut ans = None;
    for mut i in a {
        let mut cur = 1;
        while i % 2 == i / 2 % 2 {
            cur += 1;
            i /= 2;
        }
        ans.minim(cur);
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
