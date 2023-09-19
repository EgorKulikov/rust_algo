//{"name":"D. Покупка префиксов","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n5\n2\n3 4\n7\n3\n3 2 1\n2\n6\n10 6 4 6 3 4\n7\n","output":"5 0 0\n2 1\n2 2 2\n2 2 2 2 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPokupkaPrefiksov"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c = input.read_int_vec(n);
    let mut k = input.read_int();

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| (c[i], Reverse(i)));
    let mut ans = vec![0; n];
    let mut last = None;
    let mut last_c = 0;
    let mut max_ans = i32::MAX;
    for i in order {
        if last.maxim(i) {
            let delta = c[i] - last_c;
            last_c = c[i];
            let cur_ans = (k / delta).min(max_ans);
            ans[i] = cur_ans;
            k -= cur_ans * delta;
            max_ans = cur_ans;
        }
    }
    for i in (0..n - 1).rev() {
        let v = ans[i + 1];
        ans[i].maxim(v);
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
    let test_type = TestType::MultiNumber;
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
