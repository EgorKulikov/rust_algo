//{"name":"B. Выбор кубиков","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5 2 2\n4 3 3 2 3\n5 5 3\n4 2 1 3 5\n5 5 2\n5 2 4 1 3\n5 5 5\n1 2 5 4 3\n5 5 4\n3 1 2 4 5\n5 5 5\n4 3 2 1 5\n6 5 3\n1 2 3 1 2 3\n10 1 1\n1 1 1 1 1 1 1 1 1 1\n1 1 1\n42\n5 2 3\n2 2 1 1 2\n2 1 1\n2 1\n5 3 1\n3 3 2 3 2\n","output":"MAYBE\nYES\nNO\nYES\nYES\nYES\nMAYBE\nMAYBE\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BViborKubikov"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let f = input.read_size() - 1;
    let k = input.read_size();
    let a = input.read_int_vec(n);

    let mut less = 0;
    let mut less_or_equal = 0;
    for &i in &a {
        if i > a[f] {
            less += 1;
        }
        if i >= a[f] {
            less_or_equal += 1;
        }
    }
    if less >= k {
        out.print_line("NO");
        return;
    }
    if less_or_equal <= k {
        out.print_line("YES");
        return;
    }
    out.print_line("MAYBE");
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
