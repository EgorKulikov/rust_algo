//{"name":"A. Великая последовательность","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 4\n1 16 4 4\n6 2\n1 2 2 2 4 7\n5 3\n5 2 3 5 15\n9 10\n10 10 10 20 1 100 200 2000 3\n","output":"0\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVelikayaPosledovatelnost"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_long();
    let mut a = input.read_long_vec(n);

    a.sort_unstable();
    let mut qty: DefaultMap<i64, usize> = DefaultMap::new();
    let mut ans = n;
    for i in a {
        if i % x == 0 && qty[i / x] > 0 {
            ans -= 2;
            qty[i / x] -= 1;
        } else {
            qty[i] += 1;
        }
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
