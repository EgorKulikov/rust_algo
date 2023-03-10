//{"name":"Minimise Operations","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_MINOPS","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\nabcd\n4\ngijk\n4\nacbe\n","output":"0\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimiseOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s: Str = input.read();

    let mut ans = vec![n; 26];
    for i in 0..n {
        let mut c = s[i] as usize - 'a' as usize;
        c = (c + 26 - i % 26) % 26;
        ans[c] -= 1;
    }
    out_line!(ans.into_iter().min());
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
