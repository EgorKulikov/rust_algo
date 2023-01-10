//{"name":"F. Лаборатория на Плутоне","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n1\n2\n7\n","output":"1 1\n#\n1 2\n##\n2 4\n.###\n####\n"},{"input":"3 2\n1000000007\n1\n2\n7\n","output":"4 1\n6 2\n12 22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLaboratoriyaNaPlutone"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{dynamic_value, out_line};

fn bounds(n: usize) -> (usize, usize) {
    let mut a = 1;
    let mut b = 1;
    while a * b < n {
        if a == b {
            b += 1;
        } else {
            a += 1;
        }
    }
    (a, b)
}

fn solve1(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    let (a, b) = bounds(n);
    let ans = Arr2d::generate(a, b, |i, j| if i * b + j < n { '#' } else { '.' });
    out_line!(a, b);
    output().print_table(&ans);
}

fn solve2<T: DynamicValue<i32>>(input: &mut Input, q: &[ModInt<i32, T>], _test_case: usize) {
    let n = input.read_size();

    let (mut a, mut b) = bounds(n);
    let mut ans = ModInt::zero();
    while a * b >= n {
        ans += q[a * b - n];
        if a != b {
            ans += q[b * a - n];
        }
        a -= 1;
        b += 1;
    }
    out_line!(2 * (a + b), ans);
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
        TestType::Single => solve1(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            let u = input.read_size();
            match u {
                1 => {
                    for i in 0usize..t {
                        solve1(&mut input, i + 1);
                    }
                }
                2 => {
                    let m = input.read_int();
                    dynamic_value!(Module: i32 = m);
                    type Mod = ModInt<i32, Module>;
                    let mut d = Arr2d::new(700, 700, Mod::zero());
                    for i in 0..d.d1() {
                        d[(0, i)] = Mod::one();
                    }
                    for i in 1..d.d1() {
                        for j in 1..d.d2() {
                            for k in 1..=i.min(j) {
                                let c = d[(i - k, k)];
                                d[(i, j)] += c;
                            }
                        }
                    }
                    let mut q = (0..d.d1()).map(|i| d[(i, i)]).collect::<Vec<_>>();
                    // eprintln!("{:?}", q);
                    for _ in 0..2 {
                        let mut qq = vec![Mod::zero(); q.len()];
                        for i in 0..q.len() {
                            for j in 0..q.len() - i {
                                qq[i + j] += q[i] * q[j];
                            }
                        }
                        q = qq;
                    }
                    for i in 0usize..t {
                        solve2(&mut input, &q, i + 1);
                    }
                }
                _ => unreachable!(),
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve1(&mut input, i);
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
