//{"name":"D. Сбалансированный раунд","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 1\n1 2 4 5 6\n1 2\n10\n8 3\n17 3 1 20 12 5 17 12\n4 2\n2 4 6 8\n5 3\n2 3 19 10 8\n3 4\n1 10 5\n8 1\n8 3 1 4 5 10 7 3\n","output":"2\n0\n5\n0\n3\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSbalansirovanniiRaund"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_int();
    let mut a = input.read_int_vec(n);

    a.sort();
    let mut start = 0;
    let mut ans = 0;
    for (i, (&a, &b)) in a.consecutive_iter().enumerate() {
        if b - a > k {
            ans.maxim(i + 1 - start);
            start = i + 1;
        }
    }
    ans.maxim(n - start);
    out_line!(n - ans);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
