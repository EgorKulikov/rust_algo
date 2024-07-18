//{"name":"A. Разнообразная игра","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1\n1\n2 1\n2\n1\n1 5\n2 4 5 3 1\n2 4\n1 2 3 4\n5 6 7 8\n3 3\n4 2 1\n9 8 3\n6 7 5\n","output":"-1\n1\n2\n4 5 3 1 2\n6 7 8 5\n2 3 4 1\n8 3 9\n7 5 6\n2 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARaznoobraznayaIgra"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_size_table(n, m);

    if n * m == 1 {
        out.print_line(-1);
        return;
    }
    for i in a.iter_mut() {
        *i += 1;
        if *i == n * m + 1 {
            *i = 1;
        }
    }
    out.print_line(a);
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
