//{"name":"K. Preparing for a Battle","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/K","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n5 1\n2 1 1 5 3\n10 13 31 70 1\n2 2\n2 1\n100 100\n6 2\n2 3 1 6 4 5\n10 2 5 1 2 3\n4 4\n2 1 4 3\n100 100 100 100\n4 3\n2 1 1 2\n1 1 1 1\n","output":"125\n1 1 1 1 1\n0\n2 1\n3\n1 2 2 1 2 1\n0\n2 3 2 3\n0\n2 3 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KPreparingForABattle"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::random::random;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::io::{stdin, stdout, Write};

fn solve_impl(n: usize, k: usize, m: Vec<usize>, c: Vec<i64>) -> (i64, Vec<usize>) {
    if k == 1 {
        return (c.into_iter().sum::<i64>(), vec![1; n]);
    }
    if k >= 3 {
        let mut ans = vec![0; n];
        for i in 0..n {
            if ans[i] != 0 {
                continue;
            }
            let mut j = i;
            let mut next = 0;
            let mut last = 0;
            while ans[j] == 0 {
                ans[j] = next + 1;
                next = (next + 1) % 3;
                last = j;
                j = m[j];
            }
            if ans[last] == ans[j] {
                ans[last] = next + 1;
            }
        }
        return (0, ans);
    }
    let mut ans = vec![0; n];
    let mut cost = 0;
    let mut dep = vec![0; n];
    for i in 0..n {
        if ans[i] != 0 {
            continue;
        }
        let mut j = i;
        let mut cd = 0;
        while ans[j] == 0 {
            ans[j] = 3;
            dep[j] = cd;
            cd += 1;
            j = m[j];
        }
        if ans[j] == 3 {
            if cd % 2 == dep[j] % 2 {
                let mut k = j;
                let mut next = 1;
                loop {
                    ans[k] = next;
                    next = 3 - next;
                    k = m[k];
                    if k == j {
                        break;
                    }
                }
            } else {
                let mut k = j;
                let mut min_cost = None;
                let mut min_at = 0;
                loop {
                    if min_cost.minim(c[k]) {
                        min_at = k;
                    }
                    k = m[k];
                    if k == j {
                        break;
                    }
                }
                let mut k = m[min_at];
                let mut next = 1;
                loop {
                    ans[k] = next;
                    next = 3 - next;
                    k = m[k];
                    if k == m[min_at] {
                        break;
                    }
                }
                cost += min_cost.unwrap();
            }
            cd = dep[j];
        }
        let mut k = i;
        while k != j {
            if dep[k] % 2 == cd % 2 {
                ans[k] = ans[j];
            } else {
                ans[k] = 3 - ans[j];
            }
            k = m[k];
        }
    }
    (cost, ans)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size_vec(n).dec_by_one();
    let c = input.read_long_vec(n);

    let (cost, ans) = solve_impl(n, k, m.clone(), c.clone());
    let mut act_cost = 0;
    for i in 0..n {
        if ans[i] == ans[m[i]] {
            act_cost += c[i];
        }
        assert!(ans[i] >= 1 && ans[i] <= k);
    }
    if k > 2 {
        assert_eq!(act_cost, cost);
    }
    out_line!(cost);
    out_line!(ans);
}

#[test]
fn test() {
    let n = 20;
    let r = random();
    loop {
        let k = 2; //(r.next(3) + 1).into_usize();
        let m = (0..n.into_u64())
            .map(|i| {
                let res = r.next(n.into_u64() - 1);
                if res >= i { res + 1 } else { res }.into_usize()
            })
            .collect_vec();
        let c = (0..n).map(|_| r.next(1000000).into_i64() + 1).collect_vec();
        let act = solve_impl(n, k, m.clone(), c.clone());
        if act.0 == 0 {
            continue;
        }
        let mut g = vec![0; n];
        let mut rec = RecursiveFunction::new(|f, i| {
            if i == n {
                let mut cur = 0;
                for j in 0..n {
                    if g[j] == g[m[j]] {
                        cur += c[j];
                    }
                }
                return cur;
            }
            let mut res = None;
            for j in 1..=k {
                g[i] = j;
                res.minim(f.call(i + 1));
            }
            res.unwrap()
        });
        let exp = rec.call(0);
        if act.0 != exp {
            println!("n = {}", n);
            println!("k = {}", k);
            println!("m = {:?}", m);
            println!("c = {:?}", c);
            println!("exp = {}", exp);
            println!("act = {:?}", act);
            panic!();
        }
        print!(".");
        stdout().flush();
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
