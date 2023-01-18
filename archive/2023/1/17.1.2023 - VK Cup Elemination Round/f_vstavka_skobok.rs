//{"name":"F. Вставка скобок","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"1 7500\n","output":"249561089\n"},{"input":"2 6000\n","output":"519087064\n"},{"input":"5 4000\n","output":"119387743\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVstavkaSkobok"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let q = input.read_int();

    type Mod = ModIntF;
    let p = Mod::new(q) / Mod::new(10000);
    let mut res = Arr2d::new(n + 1, n + 1, None);
    let c = Combinations::<Mod>::new(n + 1);
    let mut rec = RecursiveFunction2::new(|f, bal: usize, here: usize| -> (Mod, Mod) {
        match res[(bal, here)] {
            Some(x) => x,
            None => {
                let (single, pair) = {
                    if here == 0 {
                        (Mod::one(), Mod::one())
                    } else {
                        let mut single = Mod::zero();
                        for i in 0..here {
                            let (s1, _) = f.call(bal + 1, i);
                            let (_, p2) = f.call(bal, here - i - 1);
                            single += s1 * p2 * p * c.c(here - 1, i);
                            if bal > 0 {
                                let (s1, _) = f.call(bal - 1, i);
                                let (_, p2) = f.call(bal, here - i - 1);
                                single += s1 * p2 * (Mod::one() - p) * c.c(here - 1, i);
                            }
                        }
                        let mut pair = single * Mod::new(2);
                        for i in 1..here {
                            let (s1, _) = f.call(bal, i);
                            let (s2, _) = f.call(bal, here - i);
                            pair += s1 * s2 * c.c(here, i);
                        }
                        (single, pair)
                    }
                };
                res[(bal, here)] = Some((single, pair));
                (single, pair)
            }
        }
    });
    let (mut ans, _) = rec.call(0, n);
    for i in 0..n {
        ans /= Mod::from_index(2 * i + 1);
    }
    out_line!(ans);
}

#[test]
fn test() {
    use std::collections::HashSet;

    let mut ways = HashSet::new();
    ways.insert(vec![1]);
    for x in 0..500 {
        let mut new_ways = HashSet::new();
        for way in ways {
            for i in 0..way.len() {
                let mut new_way = way.clone();
                new_way[i] += 1;
                if i + 1 < new_way.len() {
                    new_way[i + 1] += 1;
                } else {
                    new_way.push(1);
                }
                new_ways.insert(new_way);
                if i > 0 {
                    let mut new_way = way.clone();
                    new_way[i] += 1;
                    new_way[i - 1] += 1;
                    new_ways.insert(new_way);
                }
            }
        }
        ways = new_ways;
        println!("{} {}", x, ways.len());
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
