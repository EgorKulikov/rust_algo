//{"name":"E. Ожидаемые компоненты","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n1 1 1 1\n4\n1 1 2 1\n4\n1 2 1 2\n5\n4 3 2 5 1\n12\n1 3 2 3 2 1 3 3 1 3 3 2\n","output":"1\n2\n3\n5\n358642921\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EOzhidaemieKomponenti"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut q: DefaultMap<_, usize> = DefaultMap::new();
    for i in a {
        q[i] += 1;
    }
    let a = q.into_values().collect_vec();
    if a.len() == 1 {
        out_line!(1);
        return;
    }
    let g = a.iter().fold(0, |g, &a| gcd(g, a));
    type Mod = ModIntF;
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut ans = HashMap::new();
    let mut num_ways = HashMap::new();
    for i in (1..=g).rev() {
        if g % i != 0 {
            continue;
        }
        let mut ways = Mod::one();
        let mut rem = n / i;
        for &j in &a {
            ways *= c.c(rem, j / i);
            rem -= j / i;
        }
        assert_eq!(rem, 0);
        let mut res = Mod::zero();
        for &j in &a {
            if j / i > 1 {
                res += c.c(n / i - 2, j / i - 2) / c.c(n / i, j / i);
            }
        }
        res *= ways;
        res /= Mod::from_index(n / i);
        for (&a, &b) in &ans {
            if a % i == 0 {
                res -= b / Mod::from_index(a / i);
                ways -= num_ways[&a];
            }
        }
        ans.insert(i, res);
        num_ways.insert(i, ways);
    }
    let mut res = Mod::zero();
    let mut total_ways = Mod::zero();
    for (a, b) in ans {
        res += b;
        total_ways += num_ways[&a] / Mod::from_index(n / a);
    }
    res /= total_ways;
    res = Mod::from_index(n) * (Mod::one() - res);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
