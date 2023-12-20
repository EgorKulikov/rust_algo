//{"name":"E1. Игра с шариками (простая версия)","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/E1","interactive":false,"timeLimit":3500,"tests":[{"input":"5\n3\n4 2 1\n1 2 4\n4\n1 20 1 20\n100 15 10 20\n5\n1000000000 1000000000 1000000000 1000000000 1000000000\n1 1 1 1 1\n3\n5 6 5\n2 1 7\n6\n3 2 4 2 5 5\n9 4 7 9 2 5\n","output":"1\n-9\n2999999997\n8\n-6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1IgraSSharikamiProstayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by_key(|&i| Reverse(a[i] + b[i]));
    let mut ans = 0;
    for (i, j) in order.into_iter().enumerate() {
        if i % 2 == 0 {
            ans += a[j] - 1;
        } else {
            ans -= b[j] - 1;
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
