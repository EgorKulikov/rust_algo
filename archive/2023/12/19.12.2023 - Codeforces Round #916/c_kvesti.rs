//{"name":"C. Квесты","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/C","interactive":false,"timeLimit":2500,"tests":[{"input":"4\n4 7\n4 3 1 2\n1 1 1 1\n3 2\n1 2 5\n3 1 8\n5 5\n3 2 4 1 4\n2 3 1 4 7\n6 4\n1 4 5 4 5 10\n1 5 1 2 5 1\n","output":"13\n4\n15\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKvesti"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let mut ans = 0;
    let mut sum_a = 0;
    let mut max_b = 0;
    for (i, (a, b)) in a.into_iter().zip(b).enumerate() {
        max_b.maxim(b);
        sum_a += a;
        if k > i {
            ans.maxim(sum_a + max_b * (k - i - 1));
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
