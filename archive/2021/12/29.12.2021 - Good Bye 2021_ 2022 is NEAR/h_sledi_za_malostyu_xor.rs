//{"name":"H. Следи за малостью XOR","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"https://codeforces.com/contest/1616/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n0 1 2 3\n","output":"8\n"},{"input":"3 6\n4 2 2\n","output":"7\n"},{"input":"4 0\n1 1 2 2\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HSlediZaMalostyuXOR"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModIntF};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_unsigned();
    let a = input.read_unsigned_vec(n);

    let mut prefixes = Vec::with_capacity(30);
    for i in 0..=30 {
        let mut cur: DefaultMap<_, usize> = DefaultMap::new();
        for j in a.iter().cloned() {
            cur[j >> i] += 1;
        }
        prefixes.push(cur);
    }

    type Mod = ModIntF;
    let mut p = Vec::with_capacity(n + 1);
    let mut cur = Mod::one();
    for _ in 0..=n {
        p.push(cur);
        cur *= Mod::new(2);
    }
    fn double_prefix(
        mut step: usize,
        prefix1: u32,
        prefix2: u32,
        x: u32,
        p: &Vec<Mod>,
        prefixes: &Vec<DefaultMap<u32, usize>>,
    ) -> Mod {
        if prefixes[step][prefix1] == 0 || prefixes[step][prefix2] == 0 {
            return Mod::zero();
        }
        if step == 0 {
            return (p[prefixes[0][prefix1]] - Mod::one()) * (p[prefixes[0][prefix2]] - Mod::one());
        }
        step -= 1;
        if x.is_set(step) {
            let dp1 = double_prefix(step, prefix1 * 2, prefix2 * 2 + 1, x, p, prefixes);
            let dp2 = double_prefix(step, prefix1 * 2 + 1, prefix2 * 2, x, p, prefixes);
            (p[prefixes[step][prefix1 * 2]] - Mod::one())
                * (p[prefixes[step][prefix2 * 2]] - Mod::one())
                + (p[prefixes[step][prefix1 * 2 + 1]] - Mod::one())
                    * (p[prefixes[step][prefix2 * 2 + 1]] - Mod::one())
                + dp1 * dp2
                + dp1
                    * (p[prefixes[step][prefix1 * 2 + 1]] + p[prefixes[step][prefix2 * 2]]
                        - Mod::one())
                + dp2
                    * (p[prefixes[step][prefix1 * 2]] + p[prefixes[step][prefix2 * 2 + 1]]
                        - Mod::one())
        } else {
            double_prefix(step, prefix1 * 2, prefix2 * 2, x, p, prefixes)
                + double_prefix(step, prefix1 * 2 + 1, prefix2 * 2 + 1, x, p, prefixes)
        }
    }
    fn single_prefix(
        mut step: usize,
        prefix: u32,
        x: u32,
        p: &Vec<Mod>,
        prefixes: &Vec<DefaultMap<u32, usize>>,
    ) -> Mod {
        if prefixes[step][prefix] == 0 {
            return Mod::zero();
        }
        if step == 0 {
            return p[prefixes[0][prefix]] - Mod::one();
        }
        step -= 1;
        if x.is_set(step) {
            p[prefixes[step][prefix * 2]] + p[prefixes[step][prefix * 2 + 1]] - Mod::new(2)
                + double_prefix(step, prefix * 2, prefix * 2 + 1, x, p, prefixes)
        } else {
            single_prefix(step, prefix * 2, x, p, prefixes)
                + single_prefix(step, prefix * 2 + 1, x, p, prefixes)
        }
    }
    out_line!(single_prefix(30, 0, x, &p, &prefixes));
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
