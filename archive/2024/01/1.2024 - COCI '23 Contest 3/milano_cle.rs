//{"name":"#3 - Milano C.le","group":"DMOJ - COCI '23 Contest 3","url":"https://dmoj.ca/problem/coci23c3p3","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 5 2 4 1\n3 2 5 1 4\n","output":"2\n"},{"input":"5\n3 1 2 5 4\n4 2 3 1 5\n","output":"4\n"},{"input":"3\n3 2 1\n1 2 3\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MilanoCle"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(n).dec();

    let a = Permutation::new(a);
    let b = Permutation::new(b);
    let ord = (&b) * (&a.inv());
    let s = ord.to_vec();

    let mut len = Vec::new();
    for i in s {
        let pos = len.lower_bound(&i);
        if pos == len.len() {
            len.push(i);
        } else {
            len[pos] = i;
        }
    }
    out.print_line(len.len());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
