//{"name":"C. Голодные игры","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 2\n1 1 1 1\n3 2\n1 2 3\n1 6\n10\n6 3\n1 2 1 4 3 8\n5 999999999\n999999999 999999998 1000000000 1000000000 500000000\n","output":"8\n2\n0\n10\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGolodnieIgri"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    let s = a.partial_sums();
    let mut ans = n * (n + 1) / 2;
    let mut zeroes = vec![0; n + 1];
    for i in (0..n).rev() {
        if s[i] + x >= s[n] {
            continue;
        }
        let pos = s.upper_bound(&(s[i] + x));
        zeroes[i] = zeroes[pos] + 1;
        ans -= zeroes[i];
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
}
//END MAIN
