//{"name":"C. Разноцветная таблица","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 1\n1 1\n2 2\n1 2\n3 5\n3 2 4\n4 2\n1 2 1 2\n5 3\n1 2 3 2 1\n","output":"4\n4 2\n0 6 6 2 0\n8 6\n10 6 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRaznotsvetnayaTablitsa"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut first = vec![None; k];
    let mut last = vec![None; k];
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| Reverse(a[i]));
    let mut from = n;
    let mut to = 0;
    for i in order {
        from.minim(i);
        to.maxim(i);
        first[a[i]] = Some(from);
        last[a[i]] = Some(to);
    }
    let mut ans = Vec::with_capacity(k);
    for (f, l) in first.into_iter().zip(last.into_iter()) {
        if let (Some(f), Some(l)) = (f, l) {
            ans.push(2 * (l - f + 1));
        } else {
            ans.push(0);
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
