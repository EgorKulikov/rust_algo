//{"name":"C. Увеличения в группах","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 4 5\n8\n8 2 3 1 1 7 4 3\n5\n3 3 3 3 3\n1\n1\n2\n2 1\n","output":"3\n1\n0\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUvelicheniyaVGruppakh"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut min = n;
    let mut max = n;
    let mut ans = 0;
    for i in a {
        if i <= min {
            min = i;
        } else if i <= max {
            max = i;
        } else {
            min = max;
            max = i;
            ans += 1;
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
