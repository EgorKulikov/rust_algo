//{"name":"C - Almost Sorted","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n3 -1 1 -1\n","output":"2\n"},{"input":"5 1\n2 3 4 5 -1\n","output":"0\n"},{"input":"16 5\n-1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1\n","output":"794673086\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlmostSorted"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read();
    let d = input.read();
    let p = input.read_vec::<isize>(n).dec_by_one();

    type Mod = ModIntF;
    let mut free = BitSet::new(n);
    free.fill(true);
    for (i, j) in p.iter().cloned().enumerate() {
        if j != -2 {
            if ((i as isize) - j).abs() as usize > d {
                out_line!(0);
                return;
            }
            free.set(j as usize, false);
        }
    }

    let mut res = Arr2d::new(n + 1, 1 << (2 * d), None);
    let mut rec = RecursiveFunction2::new(|f, pos: usize, mask: usize| match res[(pos, mask)] {
        Some(val) => val,
        None => {
            let ans = if pos == n {
                Mod::one()
            } else {
                let c_mask = mask
                    + if pos + d < n && free[pos + d] {
                        1 << (2 * d)
                    } else {
                        0
                    };
                if p[pos] != -2 {
                    if c_mask.is_set(0) {
                        Mod::zero()
                    } else {
                        f.call(pos + 1, c_mask >> 1)
                    }
                } else {
                    if c_mask.is_set(0) {
                        f.call(pos + 1, c_mask >> 1)
                    } else {
                        let mut res = Mod::zero();
                        for i in 0..=(2 * d) {
                            if c_mask.is_set(i) {
                                res += f.call(pos + 1, c_mask.without_bit(i) >> 1);
                            }
                        }
                        res
                    }
                }
            };
            res[(pos, mask)] = Some(ans);
            ans
        }
    });
    let mut start_mask = 0;
    for i in 0..d {
        if free[i] {
            start_mask.set_bit(d + i);
        }
    }
    out_line!(rec.call(0, start_mask));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
