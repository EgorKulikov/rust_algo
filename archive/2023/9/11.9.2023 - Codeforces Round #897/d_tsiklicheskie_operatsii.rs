//{"name":"D. Циклические операции","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 3\n2 3 5 3 4\n4 2\n2 4 3 1\n1 1\n1\n3 1\n1 2 3\n5 3\n5 4 3 2 1\n6 1\n1 2 3 1 5 6\n","output":"YES\nNO\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTsiklicheskieOperatsii"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let b = input.read_size_vec(n).dec();

    if k == 1 {
        out_line!(b == (0..n).collect_vec());
        return;
    }
    let mut state = vec![0; n];
    for i in 0..n {
        let mut at = i;
        while state[at] == 0 {
            state[at] = 1;
            at = b[at];
        }
        if state[at] == 1 {
            let mut len = 0;
            while state[at] == 1 {
                state[at] = 2;
                len += 1;
                at = b[at];
            }
            if len != k {
                out_line!(false);
                return;
            }
        }
        at = i;
        while state[at] == 1 {
            state[at] = 2;
            at = b[at];
        }
    }
    out_line!(true);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
