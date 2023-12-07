//{"name":"D. Прыжки по отрезкам","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5\n1 5\n3 4\n5 6\n8 10\n0 1\n3\n0 2\n0 1\n0 3\n3\n3 8\n10 18\n6 11\n4\n10 20\n0 5\n15 17\n2 2\n","output":"7\n0\n5\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrizhkiPoOtrezkam"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let seg = input.read_long_pair_vec(n);

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let k = (left + right) / 2;
        let mut from = 0;
        let mut to = 0;
        let mut can = true;
        for &(l, r) in &seg {
            from -= k;
            to += k;
            from.maxim(l);
            to.minim(r);
            if from > to {
                can = false;
                break;
            }
        }
        if can {
            right = k;
        } else {
            left = k + 1;
        }
    }
    out.print_line(left);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
