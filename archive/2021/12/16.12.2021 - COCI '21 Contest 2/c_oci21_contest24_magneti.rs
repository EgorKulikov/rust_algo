//{"name":"COCI '21 Contest 2 #4 Magneti","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p4","interactive":false,"timeLimit":1000,"tests":[{"input":"1 10\n10\n","output":"10\n"},{"input":"4 4\n1 1 1 1\n","output":"24\n"},{"input":"3 4\n1 2 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest24Magneti"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let l: usize = input.read();
    let mut r: Vec<usize> = input.read_vec(n);

    type Mod = ModInt7;

    if n == 1 {
        out_line!(l);
        return;
    }
    r.sort_unstable();
    r.reverse();

    let mut d = Vec::new();
    for i in 0..n {
        d.push(Arr3d::new(i + 1, l - n + 1, 4, None));
    }
    let c: Combinations<Mod> = Combinations::new(l + 1);
    let mut rec =
        RecursiveFunction4::new(|f, at: usize, pairs: usize, rem: usize, edges: usize| {
            assert!(at >= pairs);
            match d[at][(pairs, rem, edges)] {
                Some(val) => val,
                None => {
                    if at == n - 1 {
                        let res = if edges == 3 && pairs == 2 || edges != 0 && pairs == 1 {
                            c.c(rem + n, n)
                        } else {
                            Mod::zero()
                        };
                        d[at][(pairs, rem, edges)] = Some(res);
                        res
                    } else {
                        let mut res = Mod::zero();
                        for i in 0..2 {
                            if !edges.is_set(i) {
                                if pairs > 0 && !edges.is_set(1 - i) || pairs > 1 {
                                    res += f.call(at + 1, pairs, rem, edges | (1 << i));
                                }
                                if rem >= r[at] - 1 {
                                    res += f.call(
                                        at + 1,
                                        pairs + 1,
                                        rem - (r[at] - 1),
                                        edges | (1 << i),
                                    );
                                }
                            }
                        }
                        let free_ends = (3 - edges).count_ones() as usize;
                        if rem >= 2 * (r[at] - 1) {
                            assert!(pairs > 0 || edges != 3);
                            res += f.call(at + 1, pairs + 1, rem - 2 * (r[at] - 1), edges)
                                * Mod::from_index(free_ends + pairs - 1);
                        }
                        if rem >= r[at] - 1 && pairs > 0 {
                            assert!(pairs + free_ends >= 2);
                            res += f.call(at + 1, pairs, rem - (r[at] - 1), edges)
                                * Mod::from_index(2 * (pairs - 1) + free_ends);
                        }
                        if pairs >= 2 && edges != 3 || pairs >= 3 {
                            res +=
                                f.call(at + 1, pairs - 1, rem, edges) * Mod::from_index(pairs - 1);
                        }
                        d[at][(pairs, rem, edges)] = Some(res);
                        res
                    }
                }
            }
        });
    out_line!(rec.call(0, 0, l - n, 0));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
