//{"name":"M. Banana Monitor","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/M","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n9 5 3 2\n2 6 8 9 6 5 4 3 6\n4 1 1 1\n1 2 0 2\n","output":"3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MBananaMonitor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x_max = input.read_int();
    let a = input.read_size();
    let c = input.read_size();
    let x = input.read_int_vec(n);

    let mut is_alarmed = false;
    let mut more = 0;
    let mut less = 0;
    let mut ans = 0;
    for i in x {
        if i > x_max {
            more += 1;
            less = 0;
        } else {
            less += 1;
            more = 0;
        }
        if more >= a {
            is_alarmed = true;
        }
        if less >= c {
            is_alarmed = false;
        }
        if is_alarmed {
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
            for i in 0usize..t {
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
