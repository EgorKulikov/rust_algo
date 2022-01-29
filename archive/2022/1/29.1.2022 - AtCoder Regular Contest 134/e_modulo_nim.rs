//{"name":"E - Modulo Nim","group":"AtCoder - AtCoder Regular Contest 134","url":"https://atcoder.jp/contests/arc134/tasks/arc134_e","interactive":false,"timeLimit":6000,"tests":[{"input":"1\n3\n","output":"1\n"},{"input":"2\n5 10\n","output":"47\n"},{"input":"20\n200 200 200 200 200 200 200 200 200 200 200 200 200 200 200 200 200 200 200 200\n","output":"273710435\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EModuloNim"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::arr4d::Arr4d;
use algo_lib::collections::arr5d::Arr5d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    type Mod = ModIntF;
    let ways = |v: &[usize]| -> Mod {
        let mut res = Mod::zero();
        for i in 0usize..1 << v.len() {
            let mut cur = Mod::one();
            for &j in &a {
                let mut ways = 0;
                for (k, &l) in v.into_iter().enumerate() {
                    if l > j {
                        break;
                    }
                    if i.is_set(k) {
                        ways += 1;
                    }
                }
                cur *= Mod::new(ways);
            }
            if (i.count_ones() % 2).into_usize() == v.len() % 2 {
                res += cur;
            } else {
                res -= cur;
            }
        }
        res
    };
    let mut ans = Mod::one();
    for &i in &a {
        ans *= Mod::from_index(i);
    }
    ans -= ways(&[1]);
    ans -= ways(&[2]);
    ans -= ways(&[4, 8]);
    let mut lose = HashSet::new();
    for i in 1..(1 << (200 / 12 + 1)) {
        if i.is_set(0) {
            continue;
        }
        let vv = (0..=200 / 12)
            .filter_map(|j| if i.is_set(j) { Some(j * 12) } else { None })
            .collect_vec();
        let mut cur_lose = true;
        for k in 1..=*vv.last().unwrap() {
            let mut v = vv.iter().map(|&i| i % k).collect_vec();
            v.sort_unstable();
            if *v.last().unwrap() == 1 {
                cur_lose = false;
                break;
            }
            if *v.last().unwrap() == 2 && !v.iter().any(|&i| i != 0 && i != 2) {
                cur_lose = false;
                break;
            }
            if !v.iter().any(|&i| i != 0 && i != 4 && i != 8)
                && v.iter().any(|&i| i == 4)
                && v.iter().any(|&i| i == 8)
            {
                cur_lose = false;
                break;
            }
            if v.iter().any(|&i| i % 12 != 0) {
                continue;
            }
            let mask = v.iter().fold(
                0,
                |mask, &i| if i == 0 { mask } else { mask.with_bit(i / 12) },
            );
            if lose.contains(&mask) {
                cur_lose = false;
                break;
            }
        }
        if cur_lose {
            // println!("{:?}", vv);
            lose.insert(i);
            ans -= ways(&vv);
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

#[test]
fn gen() {
    const N: usize = 200;
    let mut lose = Arr5d::new(
        (N + 11) / 12,
        (N + 11) / 12,
        (N + 11) / 12,
        (N + 11) / 12,
        (N + 11) / 12,
        false,
    );
    let mut v = [0; 5];
    for i in (12..N).step_by(12) {
        for j in (0..=i).step_by(12) {
            for k in (0..=j).step_by(12) {
                for m in (0..=k).step_by(12) {
                    for n in (0..=m).step_by(12) {
                        lose[(i / 12, j / 12, k / 12, m / 12, n / 12)] = true;
                        for l in 1..=i {
                            v[0] = i % l;
                            v[1] = j % l;
                            v[2] = k % l;
                            v[3] = m % l;
                            v[4] = n % l;
                            v.sort_unstable();
                            if v[4] == 1 {
                                lose[(i / 12, j / 12, k / 12, m / 12, n / 12)] = false;
                                break;
                            }
                            if v[4] == 2 && !v.iter().any(|&i| i != 0 && i != 2) {
                                lose[(i / 12, j / 12, k / 12, m / 12, n / 12)] = false;
                                break;
                            }
                            if !v.iter().any(|&i| i != 0 && i != 4 && i != 8)
                                && v.iter().any(|&i| i == 4)
                                && v.iter().any(|&i| i == 8)
                            {
                                lose[(i / 12, j / 12, k / 12, m / 12, n / 12)] = false;
                                break;
                            }
                            if v.iter().any(|&i| i % 12 != 0) {
                                continue;
                            }
                            if lose[(v[4] / 12, v[3] / 12, v[2] / 12, v[1] / 12, v[0] / 12)] {
                                lose[(i / 12, j / 12, k / 12, m / 12, n / 12)] = false;
                                break;
                            }
                        }
                        if lose[(i / 12, j / 12, k / 12, m / 12, n / 12)]
                            && i != j
                            && j != k
                            && k != m
                            && m != n
                            && n != 0
                        {
                            println!("{} {} {} {} {}", i, j, k, m, n);
                        }
                    }
                }
            }
        }
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
