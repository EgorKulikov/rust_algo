//{"name":"B. Максимальная подстрока","group":"Codeforces - CodeTON Round 3 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1750/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5\n11100\n7\n1100110\n6\n011110\n7\n1001010\n4\n1000\n1\n0\n","output":"9\n12\n16\n12\n9\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaksimalnayaPodstroka"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_vec::<char>(n);

    let mut ans = s.iter().count_eq(&&'0') * s.iter().count_eq(&&'1');
    let mut qty = 1;
    for (&a, &b) in s.consecutive_iter() {
        if a != b {
            ans.maxim(qty * qty);
            qty = 0;
        }
        qty += 1;
    }
    ans.maxim(qty * qty);
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
