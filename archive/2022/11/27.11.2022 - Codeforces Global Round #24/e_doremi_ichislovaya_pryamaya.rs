//{"name":"E. Дореми и числовая прямая","group":"Codeforces - Codeforces Global Round 24","url":"https://codeforces.com/contest/1764/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4 16\n5 3\n8 12\n10 7\n15 1\n4 16\n8 12\n10 7\n15 1\n5 3\n4 16\n10 7\n15 1\n5 3\n8 12\n4 16\n15 1\n5 3\n8 12\n10 7\n1 1000000000\n500000000 500000000\n2 1000000000\n1 999999999\n1 1\n","output":"NO\nYES\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDoremiIChislovayaPryamaya"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut k = input.read_int();
    let ab = input.read_int_pair_vec(n);

    if k <= ab[0].0 {
        out_line!(true);
        return;
    }
    if k > ab[0].0 + ab[0].1 {
        out_line!(false);
        return;
    }
    k -= ab[0].1;
    let mut order = (1..n).collect_vec();
    order.sort_by_key(|&i| Reverse(ab[i].0 + ab[i].1));
    let mut at = 0;
    let mut by_a = BTreeSet::new();
    for i in 1..n {
        by_a.insert((Reverse(ab[i].0), i));
    }
    let mut by_b = BTreeSet::new();
    loop {
        while at < n - 1 && ab[order[at]].0 + ab[order[at]].1 >= k {
            by_b.insert((Reverse(ab[order[at]].1), order[at]));
            at += 1;
        }
        if let Some(&(_, i)) = by_a.iter().next() {
            if ab[i].0 >= k {
                out_line!(true);
                return;
            }
        }
        if let Some(&(_, i)) = by_b.iter().next() {
            if ab[i].1 + ab[i].0 >= k {
                k -= ab[i].1;
                by_a.remove(&(Reverse(ab[i].0), i));
                by_b.remove(&(Reverse(ab[i].1), i));
                continue;
            }
        }
        out_line!(false);
        return;
    }
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
