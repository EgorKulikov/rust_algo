//{"name":"Let It Rip!","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/LETITRIP","interactive":false,"timeLimit":1000,"tests":[{"input":"5 9 3\nPegasus LDrago Sagittario Fireblaze Pegasus\nLDrago Leone Destroyer Kerbecs Tempo\nPegasus 22\nLDrago 20\nSagittario 13\nFireblaze 15\nLeone 18\nDestroyer 20\nKerbecs 21\nLibra 16\nTempo 22\n3 4 1\n","output":"3\n0\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LetItRip"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let b = input.read_usize();
    let q = input.read_usize();
    let aa = input.read_vec::<Str>(n);
    let bb = input.read_vec::<Str>(n);
    let d = input
        .read_vec::<(Str, u32)>(b)
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut a = aa.into_iter().map(|x| d[&x]).collect::<Vec<_>>();
    let mut b = bb.into_iter().map(|x| d[&x]).collect::<Vec<_>>();
    a.sort();
    b.sort();
    type Mod = ModInt7;
    let mut ways = Arr3d::new(n + 1, n + 1, n + 1, Mod::zero());
    let mut sum_b = Arr3d::new(n + 1, n + 1, n + 1, Mod::zero());
    let mut sum = Arr3d::new(n + 1, n + 1, n + 1, Mod::zero());
    ways[(0, 0, 0)] = Mod::one();
    sum_b[(0, 0, 0)] = Mod::one();
    sum[(0, 0, 0)] = Mod::one();
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                sum_b[(i, j, k)] = ways[(i, j, k)];
                if k > 0 {
                    let add = sum_b[(i, j, k - 1)];
                    sum_b[(i, j, k)] += add;
                }
                sum[(i, j, k)] = sum_b[(i, j, k)];
                if j > 0 {
                    let add = sum[(i, j - 1, k)];
                    sum[(i, j, k)] += add;
                }
                if i < n && j < n && k < n && a[j] > b[k] {
                    ways[(i + 1, j + 1, k + 1)] = sum[(i, j, k)];
                }
            }
        }
    }

    for _ in 0..q {
        let x = input.read_usize();
        out_line!(sum[(x, n, n)]);
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
