//{"name":"uc23_l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc23_l"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_str();
    let b = input.read_str();

    if a.len() == 1 {
        if b.len() == 1 {
            if a == b {
                out.print_line(0);
            } else {
                out.print_line(-1);
            }
        } else {
            out.print_line(-1);
        }
        return;
    }
    if b.len() == 1 {
        out.print_line(-1);
        return;
    }

    let mut ans = Vec::new();
    let mut second = a[1];
    for c in a.iter().skip(2) {
        if c == b'0' {
            if second == b'1' {
                ans.push((1, 2));
                ans.push((2, 4));
                second = b'0';
            } else {
                ans.push((2, 3));
            }
        } else {
            if second == b'0' {
                ans.push((1, 2));
            }
            ans.push((2, 3));
            ans.push((1, 2));
            ans.push((2, 5));
            second = b'0';
        }
    }

    if second == b'0' {
        ans.push((1, 2));
    }
    for c in b.iter().skip(2).rev() {
        if c == b'0' {
            ans.push((1, 2));
            ans.push((1, 2));
        } else {
            ans.push((1, 2));
            ans.push((1, 2));
            ans.push((2, 3));
        }
    }
    if b[1] == b'0' {
        ans.push((1, 2));
        ans.push((2, 3));
    }
    out.print_line(ans.len());
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
    let test_type = TestType::MultiNumber;
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
