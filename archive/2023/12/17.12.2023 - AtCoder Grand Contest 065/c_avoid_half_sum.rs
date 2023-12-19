//{"name":"C - Avoid Half Sum","group":"AtCoder - AtCoder Grand Contest 065","url":"https://atcoder.jp/contests/agc065/tasks/agc065_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 2 3\n6\n1 1 2 2 3 3\n4\n1 1 1000000000 1000000000\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAvoidHalfSum"}}}

use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted();

    if a.iter().filter(|&&x| x % 2 == 1).count() == 0 {
        out.print_line("Yes");
        return;
    }
    let odd = a.iter().filter(|&&x| x % 2 == 1).count();
    if odd < a.rev()[0] {
        out.print_line("Yes");
        return;
    }
    let mut cur = 0;
    for i in a {
        if i % 2 == 0 {
            continue;
        }
        if cur % 2 == 1 && i > cur {
            out.print_line("Yes");
            return;
        }
        cur += 1;
    }
    out.print_line("No");
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
