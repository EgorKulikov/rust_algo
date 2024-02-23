//{"name":"A. Восстановление маленькой строки","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/contest/1931/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n24\n70\n3\n55\n48\n","output":"aav\nrzz\naaa\nczz\nauz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVosstanovlenieMalenkoiStroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    if n > 52 {
        out.print((b'a' + (n - 53) as u8) as char);
        out.print_line("zz");
    } else if n > 27 {
        out.print("a");
        out.print((b'a' + (n - 28) as u8) as char);
        out.print_line("z");
    } else {
        out.print("aa");
        out.print_line((b'a' + (n - 3) as u8) as char);
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
