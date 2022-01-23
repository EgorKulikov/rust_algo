//{"name":"C. XOR-сетка","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 5\n5 1\n4\n1 14 8 9\n3 1 5 9\n4 13 11 1\n1 15 4 11\n4\n2 4 1 6\n3 7 3 10\n15 9 4 2\n12 7 15 1\n","output":"4\n9\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CXORSetka"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_table::<u32>(n, n);

    let mut w = Arr2d::new(n, n, false);
    fn mark(w: &mut Arr2d<bool>, r: usize, c: usize) {
        for cell in D4::iter(r, c, w.d1(), w.d2()) {
            if w[cell] {
                panic!("");
            }
            w[cell] = true;
        }
    }
    let mut ans = 0;
    let val = |r, c| a[(r, c)] ^ a[(r, n - c - 1)];
    for i in (1..n).step_by(4) {
        ans ^= val(0, i);
        mark(&mut w, 0, i);
    }
    for i in 1..n {
        for j in (i % 2..n).step_by(2) {
            if !w[(i, j)] {
                ans ^= val(i + 1, j);
                mark(&mut w, i + 1, j);
            }
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
