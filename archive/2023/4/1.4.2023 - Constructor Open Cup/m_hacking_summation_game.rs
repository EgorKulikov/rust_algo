//{"name":"M. Hacking Summation Game","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/M","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 1 7 3\n4\n++-\n+++\n+++\n---\n","output":"4\n1 4 3 2\n1 4 3 2\n1 4 3 2\n1 2 3 4\n"},{"input":"4\n2 2 2 2\n2\n+--\n++-\n","output":"1\n1 2 4 3\n1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MHackingSummationGame"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::{IterExt, IterPartialEqExt};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable5, RecursiveFunction5};
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::cmp::Reverse;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let k = input.read_size();
    let s = input.read_vec::<Str>(k);

    if a.iter().count_eq(&&a[0]) == n {
        out_line!(s
            .iter()
            .filter(|&x| x.iter().count_eq(&&b'+') + 1 != x.iter().count_eq(&&b'-'))
            .count());
        for _ in 0..k {
            out_line!((1..=n).collect_vec());
        }
        return;
    }
    let mut ans = vec![Vec::new(); k];

    let mut rec = RecursiveFunction5::new(
        |f,
         rem_str: Vec<usize>,
         rem_num: Vec<usize>,
         sum: i32,
         mut per: Vec<usize>,
         sgn: i32|
         -> bool {
            if rem_str.is_empty() {
                return true;
            }
            if rem_num.len() == 1 {
                if sum + sgn * a[rem_num[0]] == 0 {
                    return false;
                }
                per.push(rem_num[0] + 1);
                for i in rem_str {
                    ans[i] = per.clone();
                }
                return true;
            }
            let mut with_pl = Vec::new();
            let mut with_mi = Vec::new();
            for i in rem_str {
                if s[i][per.len()] == b'+' {
                    with_pl.push(i);
                } else {
                    with_mi.push(i);
                }
            }
            let mut vars = DefaultHashMap::<_, usize>::new();
            for &i in &rem_num {
                vars[a[i]] += 1;
            }
            let mut vars = vars.into_iter().map(|(x, y)| (Reverse(y), x)).collect_vec();
            vars.sort();
            for (_, x) in vars {
                let mut new_rem_num = Vec::with_capacity(rem_num.len() - 1);
                let mut skipped = None;
                for &i in &rem_num {
                    if a[i] != x || skipped.is_some() {
                        new_rem_num.push(i);
                    } else {
                        skipped = Some(i);
                    }
                }
                let mut new_per = per.clone();
                new_per.push(skipped.unwrap() + 1);
                if f.call(
                    with_pl.clone(),
                    new_rem_num.clone(),
                    sum + sgn * x,
                    new_per.clone(),
                    1,
                ) && f.call(with_mi.clone(), new_rem_num, sum + sgn * x, new_per, -1)
                {
                    return true;
                }
            }
            false
        },
    );
    assert!(rec.call((0..k).collect_vec(), (0..n).collect_vec(), 0, Vec::new(), 1));
    out_line!(k);
    for i in 0..k {
        out_line!(ans[i]);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
