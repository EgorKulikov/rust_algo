//{"name":"E. Ela Goes Hiking","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/E","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n4\n5\n2\n","output":"0\n250000002\n250000002\n500000004\n0\n250000002\n250000002\n250000002\n250000002\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EElaGoesHiking"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::invertable::Invertable;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    if n == 1 {
        out_line!(1);
        return;
    }

    type Mod = ModInt7;
    let mut right = FenwickTree::new(n);
    right.add(n - 1, Mod::one());
    for i in (0..n - 1).rev() {
        let add = right.get(i + 1, 2 * i + 1);
        right.add(i, add);
    }
    let mut ways = Mod::one();
    let div = Mod::new(2).power(n - 1).inv().unwrap();
    for i in 0..n {
        if i % 2 == 1 {
            ways *= Mod::new(2);
        }
        out_line!(ways * right.get(i, i + 1) * div);
    }
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

#[test]
fn test() {
    use algo_lib::numbers::num_traits::bit_ops::BitOps;
    for n in 2..=20 {
        let mut res = vec![0; n];
        for j in 0..(1 << (n - 2)) {
            let mut weight_first = 1;
            let mut first = 0;
            let mut qty = 0;
            for k in 0..n - 1 {
                if j.is_set(k) {
                    qty += 1;
                } else {
                    if weight_first > qty + 1 {
                        weight_first += qty + 1;
                    } else {
                        weight_first += qty + 1;
                        first = k + 1;
                    }
                    qty = 0;
                }
            }
            res[first] += 1;
        }
        println!("n = {}, res = {:?}", n, res);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
