//{"name":"G - Highest Ratio","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_g","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 1 4 5 3\n","output":"2.80000000\n3.33333333\n4.50000000\n5.00000000\n3.00000000\n"},{"input":"3\n999999 1 1000000\n","output":"999999.00000000\n500000.50000000\n1000000.00000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GHighestRatio"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut ans = Vec::with_capacity(n);
    let mut stack = Vec::new();
    for a in a.into_iter().rev() {
        let mut cur = a;
        let mut qty = 1;
        while let Some(&(top, top_qty)) = stack.last() {
            if top * qty <= cur * top_qty {
                break;
            }
            cur += top;
            qty += top_qty;
            stack.pop();
        }
        stack.push((cur, qty));
        ans.push(cur.into_real() / qty.into_real());
    }
    ans.reverse();
    out.print_per_line(&ans);
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
                solve(&mut input, &mut output, i, &pre_calc);
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
