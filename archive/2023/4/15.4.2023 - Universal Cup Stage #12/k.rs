//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut l = input.read_vec::<i128>(n);
    let mut r = input.read_vec::<i128>(n).inc_by_one();

    let mut ans = (r.iter().copied().min().unwrap() - l.iter().copied().max().unwrap()).max(0);
    for _ in 0..2 {
        let pos_l = |i: usize, j: usize| -> i128 {
            if l[i] >= l[j] {
                0
            } else {
                (l[j] - l[i] - 1) / (j as i128 - i as i128) + 1
            }
        };
        let pos_r = |i: usize, j: usize| -> i128 {
            if r[i] >= r[j] {
                0
            } else {
                (r[j] - r[i] - 1) / (j as i128 - i as i128) + 1
            }
        };
        let mut rs = Vec::new();
        for i in 0..n {
            loop {
                if rs.len() >= 1 {
                    let last = rs[rs.len() - 1];
                    if pos_r(last, i) == 0 {
                        rs.pop();
                        continue;
                    }
                }
                if rs.len() >= 2 {
                    let last = rs[rs.len() - 1];
                    let last2 = rs[rs.len() - 2];
                    if pos_r(last2, last) >= pos_r(last, i) {
                        rs.pop();
                        continue;
                    }
                }
                break;
            }
            rs.push(i);
        }
        let mut ls = Vec::new();
        for i in (0..n).rev() {
            loop {
                if ls.len() >= 1 {
                    let last = ls[ls.len() - 1];
                    if pos_l(i, last) == 0 {
                        ls.pop();
                        continue;
                    }
                }
                if ls.len() >= 2 {
                    let last = ls[ls.len() - 1];
                    let last2 = ls[ls.len() - 2];
                    if pos_l(last, last2) >= pos_l(i, last) {
                        ls.pop();
                        continue;
                    }
                }
                break;
            }
            ls.push(i);
        }

        let mut cur = 1;
        let mut r_pos = 0;
        let mut l_pos = 0;
        loop {
            let l_until = if l_pos + 1 < ls.len() {
                pos_l(ls[l_pos + 1], ls[l_pos])
            } else {
                std::i128::MAX
            };
            let r_until = if r_pos + 1 < rs.len() {
                pos_r(rs[r_pos], rs[r_pos + 1])
            } else {
                std::i128::MAX
            };
            let next = l_until.min(r_until);
            if next > cur {
                let cr = rs[r_pos];
                let cl = ls[l_pos];
                let cur_diff = r[cr] - l[cl] - cur * (cr.into_i128() - cl.into_i128());
                let last_diff = if next == std::i128::MAX {
                    -1
                } else {
                    r[cr] - l[cl] - (next - 1) * (cr.into_i128() - cl.into_i128())
                };

                let delta = cl.into_i128() - cr.into_i128();
                if cur_diff >= 0 && last_diff >= 0 {
                    ans += (cur_diff + last_diff) * (next - cur) / 2;
                } else if cur_diff >= 0 {
                    let step = cur_diff / (-delta);
                    let last_diff = cur_diff + step * delta;
                    ans += (cur_diff + last_diff) * (step + 1) / 2;
                } else if last_diff >= 0 {
                    let step = last_diff / delta;
                    let cur_diff = last_diff - step * delta;
                    ans += (cur_diff + last_diff) * (step + 1) / 2;
                }
            }
            if next == std::i128::MAX {
                break;
            }
            if next == l_until {
                l_pos += 1;
            }
            if next == r_until {
                r_pos += 1;
            }
            cur = next;
        }

        for i in 0..n {
            l[i] *= -1;
            r[i] *= -1;
        }
        swap(&mut l, &mut r);
    }
    out_line!(ans % 998244353);
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
    stress_test::stress_test(run, tester::check);
}
//END MAIN
