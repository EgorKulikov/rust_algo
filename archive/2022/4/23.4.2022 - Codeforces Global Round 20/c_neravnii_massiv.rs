//{"name":"C. Неравный массив","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n1 1 1 1 1\n5\n2 1 1 1 2\n6\n1 1 2 3 3 4\n6\n1 2 1 4 5 4\n","output":"2\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeravniiMassiv"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut first = None;
    let mut last = None;
    for (i, (&x, &y)) in a.consecutive_iter().enumerate() {
        if x == y {
            if first.is_none() {
                first = Some(i);
            }
            last = Some(i);
        }
    }
    let mut ans = if first.is_some() {
        last.unwrap() - first.unwrap()
    } else {
        0
    };
    if ans >= 2 {
        ans -= 1;
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
