//{"name":"F. Rare Coins","group":"Codeforces - Educational Codeforces Round 163 (Rated for Div. 2)","url":"https://codeforces.com/contest/1948/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n1 0\n0 2\n2 2\n1 1\n","output":"748683265 748683265\n"},{"input":"4 3\n2 3 4 5\n1 0 7 3\n3 3\n2 3\n1 4\n","output":"997756929 273932289 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRareCoins"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let g = a.partial_sums();
    let s = b.partial_sums();
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(s[n] + 1);
    let cc = (0..=s[n])
        .map(|i| c.c(s[n], i))
        .collect_vec()
        .partial_sums();
    let mut ans = Vec::with_capacity(q);
    let den = Mod::new(2).power(s[n]).inv().unwrap();
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        let g1 = g[r] - g[l];
        let s1 = s[r] - s[l];
        let g2 = g[n] - g1;
        let s2 = s[n] - s1;
        if s2 + g2 < g1 {
            ans.push(1.into());
            continue;
        }
        if s2 + g2 - g1 >= s1 + s2 {
            ans.push(0.into());
            continue;
        }
        ans.push((cc[s[n] + 1] - cc[s2 + g2 - g1 + 1]) * den);
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
