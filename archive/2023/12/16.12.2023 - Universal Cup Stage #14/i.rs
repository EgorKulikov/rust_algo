//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let d = (0..n)
        .map(|_| {
            let d = input.read_size_vec(6);
            let mut cur = 0;
            for i in 0..6 {
                cur.set_bit(d[i]);
            }
            cur
        })
        .collect::<Vec<_>>();

    let base = (1..1024)
        .map(|mask: usize| {
            let mut cur = 0i32;
            for &i in &d {
                if (i & mask) != 0 {
                    cur += 1;
                }
            }
            (mask, cur)
        })
        .collect_vec();

    let create = |base: &[(usize, i32)], len: i32, dd: usize| -> Option<Vec<(usize, i32)>> {
        let mut res = Vec::new();
        for &(mask, q) in base {
            let mut has = q;
            if mask.is_set(dd) {
                has -= 1;
            }
            if has < 0 {
                return None;
            }
            if has < len {
                res.push((mask, has));
            }
        }
        Some(res)
    };

    let mut ans = Vec::new();
    for i in 0.. {
        for j in 1..10 {
            let mut prefix = Str::new();
            prefix.push(b'0' + (j as u8));
            let mut rec =
                RecursiveFunction2::new(|rec, len: i32, cand: Option<Vec<(usize, i32)>>| {
                    if cand.is_none() {
                        if len == 0 {
                            ans.push(prefix.clone());
                            k -= 1;
                        } else {
                            for j in 0..10 {
                                prefix.push(b'0' + (j as u8));
                                rec.call(len - 1, None);
                                prefix.pop();
                                if k == 0 {
                                    return;
                                }
                            }
                        }
                        return;
                    }
                    let cand = cand.unwrap();
                    if cand.is_empty() {
                        return;
                    }
                    for j in 0..10 {
                        prefix.push(b'0' + (j as u8));
                        rec.call(len - 1, create(&cand, len - 1, j));
                        prefix.pop();
                        if k == 0 {
                            return;
                        }
                    }
                });
            rec.call(i, create(&base, i, j));
            if k == 0 {
                break;
            }
        }
        if k == 0 {
            break;
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
