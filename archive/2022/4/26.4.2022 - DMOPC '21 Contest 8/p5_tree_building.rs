//{"name":"P5 - Tree Building","group":"DMOJ - DMOPC '21 Contest 8","url":"https://dmoj.ca/problem/dmopc21c8p5","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 3 4\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5TreeBuilding"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(k);

    let mut b = Vec::with_capacity(k);
    b.push(0);
    for (i, &j) in a.iter().enumerate().skip(1) {
        b.push(j + (i.into_i64() - 1) * a[0]);
    }
    n -= 2;
    if n <= k * k / 2 && n * k <= 1_500_000_000 {
        let mut ans = Vec::with_capacity(n + 1);
        for i in 0..=n {
            ans.push(i.into_i64() * b[1]);
        }
        for (i, &j) in b.iter().enumerate().skip(2) {
            for k in 0..=n - i {
                let val = ans[k] + j;
                ans[k + i].minim(val);
            }
        }
        out_line!(ans[n] + 2 * a[0]);
        return;
    }
    let mut best = None;
    let mut at = 0;
    for (i, &j) in b.iter().enumerate().skip(1) {
        let cur = Rational::new(j, i.into_i64());
        if best.minim(cur) {
            at = i;
        }
    }
    let mut ans = Vec::with_capacity(at);
    for i in 0..at {
        ans.push((i.into_i64() * b[1], 0));
    }
    let limit = n.into_i64() / at.into_i64();
    for (i, &j) in b.iter().enumerate().skip(2) {
        let g = gcd(at, i);
        for k in 0..g {
            let mut cur = k;
            for _ in 0..2 * at / g {
                let next = (cur + i) % at;
                let c_times = ((cur + i) / at).into_i64();
                let val = ans[cur].0 + j - c_times * b[at];
                let times = ans[cur].1 + c_times;
                if times <= limit {
                    ans[next].minim((val, times));
                }
                cur = next;
            }
        }
    }
    out_line!(ans[n % at].0 + (n / at).into_i64() * b[at] + 2 * a[0]);
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
