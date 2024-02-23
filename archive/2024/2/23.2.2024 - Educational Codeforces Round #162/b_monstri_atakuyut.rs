//{"name":"B. Монстры атакуют!","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/B","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n3 2\n1 2 3\n-1 2 3\n2 1\n1 1\n-1 1\n4 10\n3 4 2 5\n-3 -2 1 3\n5 3\n2 1 3 2 5\n-3 -2 3 4 5\n2 1\n1 2\n1 2\n","output":"YES\nNO\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMonstriAtakuyut"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    let x = input.read_vec::<isize>(n);

    let mut total = vec![0; n];
    for i in 0..n {
        total[x[i].unsigned_abs() - 1] += a[i];
    }
    let mut sum = 0;
    for i in 0..n {
        sum += total[i];
        sum -= k;
        if sum > 0 {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
    //    tester::stress_test();
}
//END MAIN
