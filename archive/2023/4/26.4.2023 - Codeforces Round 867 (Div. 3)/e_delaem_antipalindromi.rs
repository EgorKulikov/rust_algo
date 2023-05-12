//{"name":"E. Делаем антипалиндромы","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n10\ncodeforces\n3\nabc\n10\ntaarrrataa\n10\ndcbdbdcccc\n4\nwwww\n12\ncabbaccabaac\n10\naadaaaaddc\n14\naacdaaaacadcdc\n6\nabccba\n12\ndcbcaebacccd\n","output":"0\n-1\n1\n1\n-1\n3\n-1\n2\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDelaemAntipalindromi"}}}

use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::StrReader;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s = input.read_str();

    if n % 2 == 1 {
        out_line!(-1);
        return;
    }
    let q = s
        .iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>()
        .qty_bound(26);
    if q.into_iter().max().unwrap() > n / 2 {
        out_line!(-1);
        return;
    }
    let mut q = vec![0; 26];
    for i in 0..n / 2 {
        if s[i] == s[n - i - 1] {
            q[s[i] as usize - 'a' as usize] += 1;
        }
    }
    let ans = ((q.iter().sum::<usize>() + 1) / 2).max(q.iter().copied().max().unwrap());
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
