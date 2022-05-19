//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::arr4d::Arr4d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut ans = Arr4d::new(31, 31, 31, 31, None);
    let mut rec = RecursiveFunction4::new(|f, a: usize, b: usize, c: usize, d: usize| -> usize {
        match ans[(a, b, c, d)] {
            Some(res) => res,
            None => {
                let mut res = 0;
                let mut i = 0;
                while i <= d && 10 * i < k + 10 {
                    if i * 10 + c * 5 + b * 2 + a < k {
                        i += 1;
                        continue;
                    }
                    let mut j = 0;
                    while j <= c && (j == 0 || 10 * i + 5 * j < k + 5) {
                        if i * 10 + j * 5 + b * 2 + a < k {
                            j += 1;
                            continue;
                        }
                        if 10 * i + 5 * j >= k {
                            res.maxim(1 + f.call(a, b, c - j, d - i));
                            break;
                        }
                        let rem = k - (10 * i + 5 * j);
                        let mut l = (rem / 2).min(b);
                        let mut m = rem - 2 * l;
                        if m > a {
                            m -= 1;
                            l += 1;
                        }
                        res.maxim(1 + f.call(a - m, b - l, c - j, d - i));
                        j += 1;
                    }
                    i += 1;
                }
                ans[(a, b, c, d)] = Some(res);
                res
            }
        }
    });

    for _ in 0..n {
        let a = input.read_usize();
        let b = input.read_usize();
        let c = input.read_usize();
        let d = input.read_usize();
        out_line!(rec.call(a, b, c, d));
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
