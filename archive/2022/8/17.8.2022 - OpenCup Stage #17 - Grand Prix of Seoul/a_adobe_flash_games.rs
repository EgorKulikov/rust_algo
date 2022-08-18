//{"name":"A. Adobe Flash Games","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5 3\n1 3 5\n1 1\n1 2\n1 3\n","output":"6\n"},{"input":"4 3 2\n3 3 2 1\n2 1 2\n2 2 3\n","output":"7\n"},{"input":"4 2 2\n1 2 2 1\n2 1 2\n2 3 4\n","output":"-1\n"},{"input":"2 10 0\n1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAdobeFlashGames"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let _k = input.read_usize();
    let m = input.read_usize();
    let c = input.read_usize_vec(n).dec_by_one();
    let mut r = Vec::with_capacity(m);
    for _ in 0..m {
        let mut cur = 0;
        let q = input.read_usize();
        for _ in 0..q {
            cur.set_bit(input.read_usize() - 1);
        }
        r.push(cur);
    }

    const INF: i32 = i32::MAX / 2;
    let mut ans = Arr3d::new(1 << n, 1 << m, 2, None);
    let mut rec =
        RecursiveFunction3::new(|f, reg_mask: usize, eq_mask: usize, state: usize| -> i32 {
            match ans[(reg_mask, eq_mask, state)] {
                Some(x) => x,
                None => {
                    let mut res = INF;
                    let mut ready = eq_mask == 0;
                    for i in 0..n {
                        if !reg_mask.is_set(i) && c[i] != 0 {
                            ready = false;
                            break;
                        }
                    }
                    if ready {
                        res = 0;
                    }
                    let mut covered = reg_mask;
                    for i in 0..m {
                        if eq_mask.is_set(i) {
                            covered |= r[i];
                        }
                    }
                    let mut colors: usize = 0;
                    for i in 0..n {
                        if !covered.is_set(i) {
                            colors.set_bit(c[i]);
                        }
                    }
                    if colors.count_ones() == 1 {
                        res.minim(1 + f.call(reg_mask + usize::all_bits(n) - covered, eq_mask, 0));
                    }
                    if state == 0 {
                        for i in 0..m {
                            if !eq_mask.is_set(i) {
                                res.minim(1 + f.call(reg_mask, eq_mask.with_bit(i), 0));
                            }
                        }
                    }
                    for i in 0..m {
                        if eq_mask.is_set(i) {
                            res.minim(1 + f.call(reg_mask, eq_mask.without_bit(i), 1));
                        }
                    }
                    ans[(reg_mask, eq_mask, state)] = Some(res);
                    res
                }
            }
        });
    let ans = rec.call(0, 0, 0);
    out_line!(if ans != INF { ans } else { -1 });
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
