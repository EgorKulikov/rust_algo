//{"name":"C. Дореми и строительство города","group":"Codeforces - Codeforces Global Round 24","url":"https://codeforces.com/contest/1764/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n2 2 3 1\n6\n5 2 3 1 5 2\n12\n7 2 4 9 1 4 6 3 7 4 2 3\n4\n1000000 1000000 1000000 1000000\n","output":"3\n9\n35\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoremiIStroitelstvoGoroda"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    a.sort();
    let mut ans = n / 2;
    for (i, (&a, &b)) in a.consecutive_iter().enumerate() {
        if a != b {
            ans.maxim((i + 1) * (n - i - 1));
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
