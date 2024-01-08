//{"name":"A. Обмен кошельками","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 1\n1 4\n5 3\n4 5\n11 9\n83 91\n1032 9307\n839204 7281\n1000000000 1000000000\n53110 2024\n","output":"Bob\nAlice\nBob\nAlice\nBob\nBob\nAlice\nAlice\nBob\nBob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AObmenKoshelkami"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_int();
    let b = input.read_int();

    if (a + b) % 2 == 1 {
        out.print_line("Alice");
    } else {
        out.print_line("Bob");
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
