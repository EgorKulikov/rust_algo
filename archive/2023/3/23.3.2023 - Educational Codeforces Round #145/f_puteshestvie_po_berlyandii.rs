//{"name":"F. Путешествие по Берляндии","group":"Codeforces - Educational Codeforces Round 145 (Rated for Div. 2)","url":"https://codeforces.com/contest/1809/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 5\n3 4 4\n1 2 2\n5 7\n1 3 2 5 1\n2 1 1 1 2\n4 3\n1 2 1 3\n2 2 2 2\n3 2\n2 2 2\n1 2 1\n","output":"17 19 17\n13 12 12 12 14\n14 14 14 14\n8 8 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPuteshestviePoBerlyandii"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut cost = Vec::with_capacity(2 * n);
    let mut one = 0;
    for i in (0..n).chain(0..n) {
        if b[i] == 1 {
            one = k;
        }
        let used_one = one.min(a[i]);
        cost.push(2 * a[i] - used_one);
        one -= used_one;
    }
    let s = cost.as_slice().partial_sums();
    let s2 = a
        .iter()
        .copied()
        .chain(a.iter().copied())
        .collect_vec()
        .as_slice()
        .partial_sums();
    let mut next_one = Vec::with_capacity(2 * n);
    let mut cur = None;
    for i in (0..2 * n).rev() {
        if b[i % n] == 1 {
            cur = Some(i);
        }
        next_one.push(cur);
    }
    next_one.reverse();
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(p) = next_one[i] {
            ans.push((s2[p] - s2[i]) * 2 + s[i + n] - s[p]);
        } else {
            ans.push((s2[i + n] - s2[i]) * 2);
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
