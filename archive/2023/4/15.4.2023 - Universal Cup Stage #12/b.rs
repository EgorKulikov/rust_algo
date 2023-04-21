//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::NextPermutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n);

    let mut cur = vec![None; 10000];
    let mut next = vec![None; 10000];
    fn canonize(mut x: usize) -> usize {
        let mut d = Vec::new();
        while x > 0 {
            d.push(x % 10);
            x /= 10;
        }
        d.sort();
        let mut res = 0;
        let mut ten = 1;
        for i in 0..d.len() {
            res += d[i] * ten;
            ten *= 10;
        }
        res
    }
    let mut ways = vec![Vec::new(); 10000];
    for i in 0..10000 {
        if canonize(i) == i {
            let mut x = i;
            let mut d = Vec::new();
            while x > 0 {
                d.push(x % 10);
                x /= 10;
            }
            d.sort();
            loop {
                let mut cur = 0;
                for &j in &d {
                    cur *= 10;
                    cur += j;
                }
                ways[i].push(cur);
                if !d.next_permutation() {
                    break;
                }
            }
        }
    }
    cur[canonize(x)] = Some(0);
    for i in a {
        for j in 0..10000 {
            if let Some(v) = cur[j] {
                for &k in &ways[j] {
                    if k >= i {
                        next[canonize(k - i)].maxim(v + 1);
                    }
                    next[j].maxim(v);
                }
            }
        }
        swap(&mut cur, &mut next);
    }
    let mut ans = 0;
    for i in 0..10000 {
        ans.maxim(cur[i].unwrap_or(0));
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
