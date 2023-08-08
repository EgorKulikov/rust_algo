//{"name":"C. Построение по минимумам","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 3 1\n2\n10\n4\n7 5 3 5 3 3\n5\n2 2 2 2 2 2 2 2 2 2\n5\n3 0 0 -2 0 -2 0 0 -2 -2\n","output":"1 3 3\n10 10\n7 5 3 12\n2 2 2 2 2\n0 -2 0 3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPostroeniePoMinimumam"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut b = input.read_int_vec(n * (n - 1) / 2);

    b.sort();
    let mut a = Vec::with_capacity(n);
    let mut pos = 0;
    for i in 0..n {
        a.push(b[pos.min(b.len() - 1)]);
        pos += n - 1 - i;
    }
    out_line!(a);
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
