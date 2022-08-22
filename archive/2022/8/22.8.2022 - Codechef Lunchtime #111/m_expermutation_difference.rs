//{"name":"MEX Permutation Difference","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/MEXPERMDIF","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 4\n2 2\n4 100000\n","output":"0 3 1 2\n1 2 0\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MEXPermutationDifference"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve_impl(n: usize, mut k: usize) -> Option<Vec<usize>> {
    if k > n * (n + 1) / 2 {
        // out_line!(-1);
        return None;
    }
    if k >= n {
        let mut ans = Vec::with_capacity(n + 1);
        let mut add = 0;
        let mut not_add = n;
        for i in 0..=n {
            if k >= n - i {
                k -= n - i;
                ans.push(add);
                add += 1;
            } else {
                ans.push(not_add);
                not_add -= 1;
            }
        }
        // out_line!(ans);
        return Some(ans);
    }
    if n == 1 {
        return None;
    }
    if (2 * n - k) % 3 == 0 {
        let i = (2 * n - k) / 3;
        let mut ans = vec![0; n + 1];
        ans[0] = 1;
        ans[n] = 2;
        let mut next = 3;
        for j in 1..n {
            if j != i {
                ans[j] = next;
                next += 1;
            }
        }
        return Some(ans);
    }
    if (2 * n + 1 - k) % 3 == 0 {
        let i = (2 * n + 1 - k) / 3;
        if i > 0 && i < n - 1 {
            let mut ans = vec![0; n + 1];
            ans[0] = 1;
            ans[n - 1] = 2;
            ans[n] = 3;
            let mut next = 4;
            for j in 1..n - 1 {
                if j != i {
                    ans[j] = next;
                    next += 1;
                }
            }
            return Some(ans);
        }
    }
    if (2 * n + 2 - k) % 3 == 0 {
        let i = (2 * n + 2 - k) / 3;
        if i > 0 && i < n - 2 {
            let mut ans = vec![0; n + 1];
            ans[0] = 1;
            ans[n - 2] = 2;
            ans[n] = 3;
            let mut next = 4;
            for j in 1..n {
                if j != i && j != n - 2 {
                    ans[j] = next;
                    next += 1;
                }
            }
            return Some(ans);
        }
    }
    assert!(n <= 8);
    let mut ans = (0..=n).collect_vec();
    loop {
        if check(&mut ans) == k {
            return Some(ans);
        }
        match Permutation::new(ans).next() {
            None => {
                return None;
            }
            Some(next) => {
                ans = next.to_vec();
            }
        }
    }
}

fn calc(ans: &[usize]) -> usize {
    let mut set = HashSet::new();
    let mut res = 0;
    let mut cur = 0;
    let len = ans.len() - 1;
    for &i in ans.into_iter().take(len) {
        set.insert(i);
        while set.contains(&cur) {
            cur += 1;
        }
        res += cur;
    }
    res
}

fn check(ans: &mut Vec<usize>) -> usize {
    let direct = calc(&ans);
    ans.reverse();
    let reverse = calc(&ans);
    ans.reverse();
    if direct >= reverse {
        direct - reverse
    } else {
        1000000000000
    }
}

#[test]
fn test() {
    for n in 1..=30 {
        for k in 0..=n * (n + 1) / 2 {
            let ans = solve_impl(n, k);
            if let Some(mut ans) = ans {
                let val = check(&mut ans);
                if val != k {
                    println!("Error: {} {} {}", n, k, val);
                }
            } else {
                println!("{} {}", n, k);
            }
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    out_line!(solve_impl(n, k));
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
