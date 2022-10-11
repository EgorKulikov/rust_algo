//{"name":"D. Одинаковые бинарные подпоследовательности","group":"Codeforces - Codeforces Round #825 (Div. 2)","url":"https://codeforces.com/contest/1736/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1010\n3\n100010\n2\n1111\n2\n1110\n","output":"0\n1 2\n2 3 5\n1 2 5\n3 2 3 4\n1 4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DOdinakovieBinarniePodposledovatelnosti"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();

    let mut ans = Vec::new();
    let mut first_need = None;
    let mut cur_have = None;
    for i in 0..n {
        if s[2 * i] != s[2 * i + 1] {
            match cur_have {
                None => {
                    cur_have = Some(s[2 * i]);
                    ans.push(2 * i + 1);
                    first_need = Some(s[2 * i + 1]);
                }
                Some(c) => {
                    if c == s[2 * i] {
                        ans.push(2 * i + 2);
                        cur_have = Some(s[2 * i + 1]);
                    } else {
                        ans.push(2 * i + 1);
                        cur_have = Some(s[2 * i]);
                    }
                }
            }
        }
    }
    if first_need != cur_have {
        out_line!(-1);
    } else {
        out_line!(ans.len(), ans);
        out_line!((1..2 * n).step_by(2).collect_vec());
    }
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
