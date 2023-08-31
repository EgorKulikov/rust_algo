//{"name":"G. Swaps","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1 2\n","output":"2\n"},{"input":"4\n2 1 4 3\n","output":"4\n"},{"input":"6\n2 3 1 1 1 2\n","output":"18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSwaps"}}}

use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    type Mod = ModInt7;
    let mut ans = Mod::one();
    let mut done = vec![0; n];
    let inc = a.qty_bound(n);
    for i in 0..n {
        if done[i] != 0 {
            continue;
        }
        let mut cur = i;
        while done[cur] == 0 {
            done[cur] = 1;
            cur = a[cur];
        }
        if done[cur] == 1 {
            let mut mult = Mod::one();
            let mut sum = Mod::zero();
            while done[cur] == 1 {
                done[cur] = 2;
                mult *= Mod::from_index(inc[cur] + 1);
                sum += Mod::from_index(inc[cur]);
                cur = a[cur];
            }
            ans *= mult - sum;
        }
        let mut cur = i;
        while done[cur] == 1 {
            done[cur] = 2;
            ans *= Mod::from_index(inc[cur] + 1);
            cur = a[cur];
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
