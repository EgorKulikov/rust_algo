//{"name":"D. Три активности","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 10 1\n10 1 1\n1 1 10\n4\n30 20 10 1\n30 5 15 20\n30 25 10 10\n10\n5 19 12 3 18 18 6 17 10 13\n15 17 19 11 16 3 11 17 17 17\n1 17 18 10 15 8 17 3 13 12\n10\n17 5 4 18 12 4 11 2 16 16\n8 4 14 19 3 12 6 7 5 16\n3 4 8 11 10 8 10 2 20 3\n","output":"30\n75\n55\n56\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTriAktivnosti"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);
    let c = input.read_size_vec(n);

    let mut order = (0..n).collect_vec();
    let mut poi = Vec::new();
    order.sort_by_key(|&i| Reverse(a[i]));
    poi.extend_from_slice(&order[..3]);
    order.sort_by_key(|&i| Reverse(b[i]));
    poi.extend_from_slice(&order[..3]);
    order.sort_by_key(|&i| Reverse(c[i]));
    poi.extend_from_slice(&order[..3]);
    poi.sort();
    poi.dedup();
    let mut ans = 0;
    for &i in &poi {
        for &j in &poi {
            for &k in &poi {
                if i != j && j != k && i != k {
                    ans.maxim(a[i] + b[j] + c[k]);
                }
            }
        }
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
    let test_type = TestType::MultiNumber;
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
