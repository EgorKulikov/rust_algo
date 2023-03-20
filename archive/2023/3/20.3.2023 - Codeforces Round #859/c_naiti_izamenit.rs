//{"name":"C. Найти и заменить","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n7\nabacaba\n2\naa\n1\ny\n4\nbkpt\n6\nninfia\n6\nbanana\n10\ncodeforces\n8\ntestcase\n","output":"YES\nNO\nYES\nYES\nNO\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNaitiIZamenit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_size();
    let s: Str = input.read();

    let mut odd = 0;
    let mut even = 0;
    for (i, c) in s.iter().enumerate() {
        let id = (c - b'a').into_usize();
        if i % 2 == 0 {
            even.set_bit(id);
        } else {
            odd.set_bit(id);
        }
    }
    out_line!((even & odd) == 0);
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
