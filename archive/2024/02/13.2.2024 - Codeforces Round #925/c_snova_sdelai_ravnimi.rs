//{"name":"C. Снова сделай равными","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/contest/1931/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n6\n1 2 3 4 5 1\n7\n1 1 1 1 1 1 1\n8\n8 8 8 1 2 8 8 8\n1\n1\n2\n1 2\n3\n1 2 3\n7\n4 3 2 7 1 1 3\n9\n9 9 2 9 2 5 5 5 3\n","output":"4\n0\n2\n0\n1\n2\n6\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSnovaSdelaiRavnimi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let front = a.iter().take_while(|&&x| x == a[0]).count();
    let back = a.iter().rev().take_while(|&&x| x == a[n - 1]).count();
    if front == n {
        out.print_line(0);
    } else if a[0] == a[n - 1] {
        out.print_line(n - front - back);
    } else {
        out.print_line(n - front.max(back));
    }
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
