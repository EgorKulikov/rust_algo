//{"name":"B. Восстановление И","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1\n3\n2 0\n4\n1 2 3\n5\n3 5 4 2\n","output":"5 3\n3 2 1\n-1\n3 7 5 6 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVosstanovlenieI"}}}

use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let b = input.read_unsigned_vec(n - 1);

    let mut a = vec![b[0]];
    for (&i, &j) in b.consecutive_iter() {
        a.push(i | j);
    }
    a.push(b.backward()[0]);
    for (i, (&j, &k)) in b.into_iter().zip(a.consecutive_iter()) {
        if i != (j & k) {
            out.print_line(-1);
            return;
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
