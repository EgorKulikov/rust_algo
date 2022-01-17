//{"name":"#3 - Anagramistica","group":"DMOJ - COCI '20 Contest 6","url":"https://dmoj.ca/problem/coci20c6p3","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1\novo\nono\nvoo\n","output":"2\n"},{"input":"5 2\ntrava\nvatra\nvrata\nleo\nole\n","output":"3\n"},{"input":"6 3\nmali\nlima\nimal\nje\nsve\nej\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Anagramistica"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut map: DefaultMap<_, usize> = DefaultMap::new();
    for _ in 0..n {
        let mut s: Str = input.read();
        s.as_slice_mut().sort_unstable();
        map[s] += 1;
    }

    type Mod = ModInt7;
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut ways = vec![Mod::zero(); k + 1];
    ways[k] = Mod::one();
    for &i in map.values() {
        for j in 0..=k {
            if ways[j] == Mod::zero() {
                continue;
            }
            for l in 2..=i {
                let delta = l * (l - 1) / 2;
                if j >= delta {
                    let cur = c.c(i, l) * ways[j];
                    ways[j - delta] += cur;
                } else {
                    break;
                }
            }
            ways[j] *= Mod::from_index(i + 1);
        }
    }
    out_line!(ways[0]);
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
