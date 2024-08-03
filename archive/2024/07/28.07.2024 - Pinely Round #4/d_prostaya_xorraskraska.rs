//{"name":"D. Простая XOR раскраска","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n6\n","output":"1\n1\n2\n1 2\n2\n1 2 2\n3\n1 2 2 3\n3\n1 2 2 3 3\n4\n1 2 2 3 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProstayaXORRaskraska"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    match n {
        1 => {
            out.print_line(1);
            out.print_line(1);
        }
        2 => {
            out.print_line(2);
            out.print_line("1 2");
        }
        3 => {
            out.print_line(2);
            out.print_line("1 2 2");
        }
        4 => {
            out.print_line(3);
            out.print_line("1 2 2 3");
        }
        5 => {
            out.print_line(3);
            out.print_line("1 2 2 3 3");
        }
        n => {
            out.print_line(4);
            let mut ans = Vec::with_capacity(n);
            for i in 0..n {
                ans.push(i % 4 + 1);
            }
            out.print_line(ans);
        }
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
}
//END MAIN
