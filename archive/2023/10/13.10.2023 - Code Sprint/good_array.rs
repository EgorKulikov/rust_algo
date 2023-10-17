//{"name":"Good Array","group":"CodeChef - COSS2023","url":"https://www.codechef.com/COSS2023/problems/GDARY","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n-10\n5\n0 -1 0 -2 0\n4\n-1 -2 -3 -4\n","output":"Yes\n0\nNo\nYes\n0 -1 -2 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GoodArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut vars = Vec::with_capacity(2);
    let mut v1 = Vec::with_capacity(n);
    for i in 0..(n + 1).upper_div(2) {
        v1.push(-(i as i32));
    }
    for i in (0..(n - 1) / 2).rev() {
        v1.push(-(i as i32));
    }
    vars.push(v1);
    if n % 2 == 0 {
        let mut v2 = Vec::with_capacity(n);
        for i in 0..n / 2 - 1 {
            v2.push(-(i as i32));
        }
        for i in (0..=n / 2).rev() {
            v2.push(-(i as i32));
        }
        vars.push(v2);
    }
    out.set_bool_output(BoolOutput::YesNo);
    for v in vars {
        let mut good = true;
        for (&i, &j) in a.iter().zip(v.iter()) {
            if i > j {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(true);
            out.print_line(v);
            return;
        }
    }
    out.print_line(false);
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
