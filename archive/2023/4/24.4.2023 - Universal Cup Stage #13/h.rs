//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::primes::Factorize;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long() + 2;
    let k = input.read_long();

    let d = n.prime_divisors();
    let mut v = Vec::new();
    for i in 0..(1 << d.len()) {
        let mut p = 1;
        let mut q = 1;
        for j in 0..d.len() {
            if i.is_set(j) {
                p *= d[j].0;
                q *= -1;
            }
        }
        v.push((p, q));
    }
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        let mut sum = 0;
        for &(p, q) in &v {
            sum += q * ((mid - 1) / p);
        }
        if sum >= k {
            right = mid - 1;
        } else if sum < k - 1 {
            left = mid + 1;
        } else {
            if gcd(mid, n) == 1 {
                left = mid;
                break;
            }
            left = mid + 1;
        }
    }
    if left == n {
        out_line!(-1);
        return;
    }
    let mut z = n - left;
    let mut o = left;
    let mut ans = Vec::new();
    while o != 1 {
        ans.push(z / o);
        z %= o;
        swap(&mut z, &mut o);
    }
    ans.push(z - 1);
    if ans[0] == 0 {
        out_line!(ans.len() - 1, 1);
        out_line!(&ans[1..]);
    } else {
        out_line!(ans.len(), 0);
        out_line!(ans);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
