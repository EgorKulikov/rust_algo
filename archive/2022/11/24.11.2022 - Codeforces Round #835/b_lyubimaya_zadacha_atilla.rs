//{"name":"B. Любимая задача Atilla","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\na\n4\ndown\n10\ncodeforces\n3\nbcf\n5\nzzzzz\n","output":"1\n23\n19\n6\n26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLyubimayaZadachaAtilla"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_vec::<char>(n);

    let mut ans = 0;
    for c in s {
        ans.maxim(c as usize - 'a' as usize + 1);
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
