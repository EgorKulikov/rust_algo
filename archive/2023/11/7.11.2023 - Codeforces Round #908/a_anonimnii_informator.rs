//{"name":"A. Анонимный информатор","group":"Codeforces - Codeforces Round 908 (Div. 1)","url":"https://codeforces.com/contest/1893/problem/A","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n5 3\n4 3 3 2 3\n3 100\n7 2 1\n5 5\n6 1 1 1 1\n1 1000000000\n1\n8 48\n9 10 11 12 13 14 15 8\n2 1\n1 42\n","output":"Yes\nYes\nNo\nYes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnonimniiInformator"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut pos = n - 1;
    let mut was_pos = BitSet::new(n);
    for _ in 0..k {
        if was_pos[pos] {
            out.print_line(true);
            return;
        }
        was_pos.set(pos);
        if a[pos] >= n {
            out.print_line(false);
            return;
        }
        pos = (n + pos - a[pos] - 1) % n;
    }
    out.print_line(true);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    output.set_bool_output(BoolOutput::YesNo);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
