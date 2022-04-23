//{"name":"F. Подсчет перестановок","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n10 6 4\n7 3 1\n163316 11 7\n136373 11 1\n325902 11 11\n","output":"465\n12\n986128624\n7636394\n57118194\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPodschetPerestanovok"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::arr4d::Arr4d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type Mod = ModIntF;

fn solve(input: &mut Input, ways: &Arr4d<Mod>, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let x = input.read_usize();

    if k < x {
        out_line!(0);
        return;
    }
    fn c(n: usize, k: usize) -> Mod {
        let mut res = Mod::one();
        for i in n - k + 1..=n {
            res *= Mod::from_index(i);
        }
        for i in 1..=k {
            res /= Mod::from_index(i);
        }
        res
    }
    let mut ans = Mod::zero();
    for runs in 1..=x {
        for len in 2 * runs..=(k + runs).min(n) {
            if ways[(runs, len, x, k)] == Mod::zero() {
                continue;
            }
            ans += c(n - len + runs, runs) * ways[(runs, len, x, k)];
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut runs = Arr3d::new(13, 12, 12, Mod::zero());
    for i in 2..=12 {
        let mut ways = Arr4d::new(1 << i, i, i, 12, Mod::zero());
        for j in 1..i {
            ways[(1 << j, j, 0, j)] = Mod::one();
        }
        for j in 0..(1 << i) {
            for k in 0..i {
                for l in 0..i {
                    for m in 0..12 {
                        if ways[(j, k, l, m)] == Mod::zero() {
                            continue;
                        }
                        for n in 0..i {
                            if j.is_set(n) {
                                continue;
                            }
                            let mask = j.with_bit(n);
                            if mask == usize::all_bits(mask.count_ones().into_usize())
                                && mask != usize::all_bits(i)
                            {
                                continue;
                            }
                            let nl = l + if n < k { 1 } else { 0 };
                            let nm = m + n - (j & usize::all_bits(n)).count_ones().into_usize();
                            if nm >= 12 {
                                continue;
                            }
                            let w = ways[(j, k, l, m)];
                            ways[(mask, n, nl, nm)] += w;
                        }
                    }
                }
            }
        }
        for k in 0..i {
            for l in 0..i {
                for m in 0..12 {
                    runs[(i, l, m)] += ways[(usize::all_bits(i), k, l, m)];
                }
            }
        }
    }

    let mut ways = Arr4d::new(12, 23, 12, 12, Mod::zero());
    ways[(0, 0, 0, 0)] = Mod::one();
    for i in 0..11 {
        for j in 0..23 {
            for k in 0..12 {
                for l in 0..12 {
                    if ways[(i, j, k, l)] == Mod::zero() {
                        continue;
                    }
                    for len in 2..=12 {
                        for a in 1..12 - k {
                            for b in 1..12 - l {
                                if runs[(len, a, b)] != Mod::zero() {
                                    let w = ways[(i, j, k, l)] * runs[(len, a, b)];
                                    ways[(i + 1, j + len, k + a, b + l)] += w;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &ways, i + 1);
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
