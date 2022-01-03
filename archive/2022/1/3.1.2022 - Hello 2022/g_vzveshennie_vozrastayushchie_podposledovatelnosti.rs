//{"name":"G. Взвешенные возрастающие подпоследовательности","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n6 4 8 6 5\n4\n1 2 3 4\n3\n3 2 2\n4\n4 5 6 5\n","output":"4\n12\n0\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GVzveshennieVozrastayushchiePodposledovatelnosti"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{compress, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    type Mod = ModInt7;
    let mut order = a.iter().enumerate().map(|(i, a)| (-*a, i)).collect_vec();
    order.sort();
    let mut ft = FenwickTree::new(n);
    let mut ans = vec![Mod::one(); n];
    let mut right = None;
    let mut added = Vec::new();
    for (_, i) in order.iter().cloned() {
        let cur = match right {
            None => {
                right = Some(i);
                added.push(i);
                Mod::zero()
            }
            Some(r) => {
                if r > i {
                    let res = ft.get(i, n) + Mod::one();
                    ft.add(i, res);
                    added.push(i);
                    res
                } else {
                    let (c, (a,)) = compress!(added);
                    let mut lft = FenwickTree::new(c.len());
                    lft.add(a[0], Mod::one());
                    ft.add(r, Mod::one());
                    for j in a.into_iter().skip(1) {
                        let cur = lft.get(j, c.len());
                        ft.add(c[j], cur);
                        lft.add(j, cur);
                    }
                    right = Some(i);
                    added.clear();
                    added.push(i);
                    Mod::zero()
                }
            }
        };
        ans[i] *= cur;
    }
    order.reverse();
    ft.clear();
    for (_, i) in order {
        let cur = ft.get(0, i) + Mod::one();
        ft.add(i, cur);
        ans[i] *= cur;
    }
    out_line!(ans.into_iter().fold(Mod::zero(), |a, b| a + b));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
