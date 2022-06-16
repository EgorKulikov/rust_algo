//{"name":"B. Железнодорожная система","group":"Codeforces - Codeforces Round #796 (Div. 1)","url":"https://codeforces.com/contest/1687/problem/B","interactive":true,"timeLimit":1000,"tests":[{"input":"5 4\n\n0\n\n5\n\n9\n\n7\n","output":"? 0000\n\n? 1110\n\n? 1111\n\n? 1101\n\n! 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZheleznodorozhnayaSistema"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_usize();
    let m = input.read_usize();

    let mut s: Str = vec![b'0'; m].into();
    let mut len = Vec::with_capacity(m);
    for i in 0..m {
        s[i] = b'1';
        out_line!('?', s);
        s[i] = b'0';
        len.push(input.read_usize());
    }
    let mut order = (0..m).collect_vec();
    order.sort_by_key(|&i| len[i]);
    let mut cur = 0;
    for i in order {
        s[i] = b'1';
        out_line!('?', s);
        if cur + len[i] == input.read_usize() {
            cur += len[i];
        } else {
            s[i] = b'0';
        }
    }
    out_line!('!', cur);
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
