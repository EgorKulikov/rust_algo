//{"name":"Chef and Spice Grid","group":"CodeChef - START66","url":"https://www.codechef.com/START66A/problems-old/CHEFSPICE","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 5 13\n3627 17424 27008005\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ChefAndSpiceGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::primes::Factorize;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let m = input.read_long();
    let k = input.read_long();

    if n * m < k {
        out_line!(false);
        return;
    }
    let nn = n * m - k;
    if nn <= n.max(m) || k == 0 {
        out_line!(true);
        return;
    }
    if k < n.min(m) {
        out_line!(false);
        return;
    }
    let min = (nn + n.max(m) - 1) / n.max(m);
    let max = n.min(m);
    if max - min < 5000 {
        for i in min..=max {
            if nn % i == 0 {
                out_line!(true);
                return;
            }
        }
        out_line!(false);
        return;
    }
    let pd = (nn).prime_divisors();
    if let Some(&(p, _)) = pd.last() {
        if p > n.max(m) {
            out_line!(false);
            return;
        }
    }
    let mut rec = RecursiveFunction2::new(|f, mut d: i64, step: usize| -> bool {
        if d > max {
            return false;
        }
        if d >= min {
            return true;
        }
        if step == pd.len() {
            return false;
        } else {
            let (p, e) = pd[step];
            for i in 0..=e {
                if f.call(d, step + 1) {
                    return true;
                }
                if i < e {
                    d *= p;
                }
            }
            return false;
        }
    });
    out_line!(rec.call(1, 0));
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
