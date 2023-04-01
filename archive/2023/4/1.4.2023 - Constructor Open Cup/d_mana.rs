//{"name":"D. Mana","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n25\n69\n1337\n576460752303423487\n","output":"21\n2211\n211121\n2121122211\n2222222222222222222222222222222222222222222222222222222222\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMana"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_long();

    let mut ans = Str::new();
    while n > 1 {
        if (n - 1) / 2 % 2 == 1 {
            ans.push(b'2');
            n = (n - 1) / 2;
        } else {
            ans.push(b'1');
            n = (n + 1) / 2;
        }
    }
    ans.reverse();
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
