//{"name":"D1. Горячий запуск (простая версия)","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/D1","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n3 2\n1 2 2\n3 2\n2 1\n4 2\n1 2 1 2\n5 3\n2 1\n4 3\n1 2 3 1\n100 100 100\n1 1 1\n5 2\n2 1 2 1 1\n65 45\n54 7\n5 3\n1 3 2 1 2\n2 2 2\n1 1 1\n5 1\n1 1 1 1 1\n1000000000\n999999999\n5 6\n1 6 1 4 1\n3 6 4 1 4 5\n1 1 1 1 4 1\n1 3\n3\n4 5 6\n1 2 3\n8 3\n3 3 3 1 2 3 2 1\n10 10 8\n10 10 5\n","output":"6\n11\n301\n225\n8\n4999999996\n11\n6\n63\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1GoryachiiZapuskProstayaVersiya"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec_by_one();
    let cold = input.read_long_vec(k);
    let hot = input.read_long_vec(k);

    let mut base = 0;
    let mut n_a = Vec::new();
    for i in 0..n {
        if i > 0 && a[i] == a[i - 1] {
            base += hot[a[i]];
        } else {
            base += cold[a[i]];
            n_a.push(a[i]);
        }
    }
    let a = n_a;
    let delta = hot
        .iter()
        .zip(cold.iter())
        .map(|(h, c)| c - h)
        .collect_vec();
    let mut ans = Vec::with_capacity(a.len() + 1);
    let mut last = Vec::with_capacity(a.len());
    let mut at = vec![None; k];
    for (i, &a) in a.iter().enumerate() {
        last.push(at[a]);
        at[a] = Some(i);
    }
    ans.push(0);
    for (i, a) in a.into_iter().enumerate() {
        let mut cand = ans[i];
        if let Some(j) = last[i] {
            cand = cand.max(ans[j + 2] + delta[a]);
        }
        ans.push(cand);
    }
    out_line!(base - *ans.last().unwrap());
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
