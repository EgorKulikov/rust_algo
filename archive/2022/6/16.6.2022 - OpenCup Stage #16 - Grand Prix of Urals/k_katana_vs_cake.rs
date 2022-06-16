//{"name":"K. Katana vs. Cake","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/K/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1.0 2.0 1.0 0.7\n-0.6 0.0 1.0 0.2\n1.0 -3.0 1.0 0.5\n","output":"1.05139753126103951963\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKatanaVsCake"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut planes: Vec<(f64, f64, f64, f64)> = input.read_vec(n);

    const BUBEN: usize = 2600;
    // const BUBEN2: usize = 1000000000000000;
    let step = 2. / (BUBEN as f64);
    // let step2 = 2. / (BUBEN2 as f64);
    let shift = step / 2.;
    // let shift2 = step2 / 2.;
    let mut ans = vec![0f64; 1 << n];
    let cube = step * step;
    let mut val = vec![0f64; n];
    for i in 0..n {
        let (a, b, c, _) = &mut planes[i];
        let cos = 2f64 / 3.;
        let sin = (1. - cos * cos).sqrt();
        let na = cos * *a + sin * *b;
        let nb = -sin * *a + cos * *b;
        *a = na;
        *b = nb;
        let na = cos * *a + sin * *c;
        let nc = -sin * *a + cos * *c;
        *a = na;
        *c = nc;
    }
    for i in 0..BUBEN {
        let i = i as f64;
        let x = i * step + shift - 1.;
        for j in 0..BUBEN {
            let j = j as f64;
            let y = j * step + shift - 1.;
            for k in 0..n {
                let (a, b, _, d) = planes[k];
                val[k] = a * x + b * y + d;
            }
            let mut from = -(1. - x * x - y * y).sqrt();
            let to = -from;
            // let mask = |m: f64| -> usize {
            //     let z = m * step2 + shift2 - 1.;
            //     let mut res = 0;
            //     for i in 0..n {
            //         let (_, _, c, _) = planes[i];
            //         if val[i] + c * z >= 0. {
            //             res.set_bit(i);
            //         }
            //     }
            //     res
            // };
            while from < to {
                let mut until = to;
                let mut mask = 0;
                for i in 0..n {
                    let (_, _, c, _) = planes[i];
                    let c_val = val[i] + c * from;
                    if c_val > 0. {
                        mask.set_bit(i);
                        if c < 0. {
                            let zz = from - c_val / c;
                            if zz == from {
                                mask.unset_bit(i);
                            } else {
                                until.minim(zz);
                            }
                        }
                    } else {
                        if c > 0. {
                            let zz = from - c_val / c;
                            if zz == from {
                                mask.set_bit(i);
                            } else {
                                until.minim(zz);
                            }
                        }
                    }
                }
                ans[mask] += until - from;
                from = until;
            }
        }
    }
    let mut res = 0.;
    for c in ans {
        if c > res {
            res = c;
        }
    }
    out_line!(res * cube);
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
