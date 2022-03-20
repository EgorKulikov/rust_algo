//{"name":"B. Префиксные удаления","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"https://codeforces.com/contest/1654/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\nabcabdc\na\nbbbbbbbbbb\ncodeforces\ncffcfccffccfcffcfccfcffccffcfccf\nzyzyzwxxyyxxyyzzyzzxxwzxwywxwzxxyzzw\n","output":"abdc\na\nb\ndeforces\ncf\nxyzzw\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPrefiksnieUdaleniya"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut was = BitSet::new(256);
    let mut start = s.len();
    for (i, c) in s.iter().enumerate().rev() {
        if !was[c.into_usize()] {
            was.set(c.into_usize(), true);
            start = i;
        }
    }
    out_line!(Str::U8(&s[start..]));
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
