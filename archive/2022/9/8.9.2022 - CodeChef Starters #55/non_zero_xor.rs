//{"name":"Non Zero Xor","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/NZXOR","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n1 2 3 4 4\n3\n0 0 0\n6\n2 3 5 7 11 13\n","output":"2\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NonZeroXor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut has = HashSet::new();
    has.insert(0);
    let mut cur = 0;
    let mut ans = 0;
    for i in a {
        cur ^= i;
        if has.contains(&cur) {
            ans += 1;
            has.clear();
            cur = 0;
        }
        has.insert(cur);
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
