//{"name":"I. Imbalance","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/I/","interactive":false,"timeLimit":1000,"tests":[{"input":"7 0\n","output":"1\n"},{"input":"9 2\n","output":"4\n"},{"input":"20 4\n","output":"480\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IImbalance"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable5, RecursiveFunction5};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    type Mod = ModInt7;
    let c: Combinations<Mod> = Combinations::new(300_001);

    let mut ans = Mod::zero();
    for h in 1..=20 {
        let full = (1 << h) - 1;
        if n > full {
            continue;
        }
        let delta = full - n;
        let mut rec = RecursiveFunction5::new(
            |f, h: usize, cur: usize, next: usize, delta: usize, k: usize| -> Mod {
                if delta == 0 && k == 0 {
                    return Mod::one();
                }
                if h == 0 {
                    return Mod::zero();
                }
                if k > delta
                    || (delta >> (h - 1))
                        + (delta & usize::all_bits(h - 1)).count_ones().into_usize()
                        > k
                {
                    return Mod::zero();
                }
                let mut res = Mod::zero();
                for i in 0..=cur.min(k) {
                    if delta < i << (h - 1) {
                        break;
                    }
                    res += f.call(h - 1, 2 * cur - i + next, i, delta - (i << (h - 1)), k - i)
                        * c.c(cur, i);
                }
                res
            },
        );
        ans += rec.call(h - 1, 1, 0, delta, k);
    }
    out_line!(ans * Mod::new(2).power(k));
}

#[test]
fn test() {
    use algo_lib::collections::arr2d::Arr2d;
    let mut ans = Arr2d::new(10, 21, Vec::new());
    ans[(0, 0)] = vec![1i64];
    ans[(1, 0)] = vec![0, 1];
    for i in 2..10 {
        for j in 0..=20 {
            for k in 0..=20 {
                if j + k > 20 {
                    continue;
                }
                if ans[(i - 1, j)].len() == 0 || ans[(i - 1, k)].len() == 0 {
                    continue;
                }
                if ans[(i, j + k)].len() < ans[(i - 1, j)].len() + ans[(i - 1, k)].len() {
                    let len = ans[(i - 1, j)].len() + ans[(i - 1, k)].len();
                    ans[(i, j + k)].resize(len, 0);
                }
                for l in 0..ans[(i - 1, j)].len() {
                    for m in 0..ans[(i - 1, k)].len() {
                        let add = ans[(i - 1, j)][l] * ans[(i - 1, k)][m];
                        ans[(i, j + k)][l + m + 1] += add;
                    }
                }
            }
        }
        for j in 0..20 {
            for k in 0..20 {
                if j + k > 19 {
                    continue;
                }
                if ans[(i - 1, j)].len() == 0 || ans[(i - 2, k)].len() == 0 {
                    continue;
                }
                if ans[(i, j + k + 1)].len() < ans[(i - 1, j)].len() + ans[(i - 2, k)].len() {
                    let len = ans[(i - 1, j)].len() + ans[(i - 2, k)].len();
                    ans[(i, j + k + 1)].resize(len, 0);
                }
                for l in 0..ans[(i - 1, j)].len() {
                    for m in 0..ans[(i - 2, k)].len() {
                        let add = ans[(i - 1, j)][l] * ans[(i - 2, k)][m];
                        ans[(i, j + k + 1)][l + m + 1] += add;
                    }
                }
            }
        }
        for j in 0..=20 {
            println!("{i} {j} --> {:?}", ans[(i, j)]);
        }
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
