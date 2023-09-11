//{"name":"C. salyg1n и игра с MEX","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/C","interactive":true,"timeLimit":3000,"tests":[{"input":"3\n5\n1 2 3 5 7\n\n7\n\n5\n\n-1\n\n3\n0 1 2\n\n0\n\n-1\n\n3\n5 7 57\n\n-1\n","output":"8\n\n57\n\n0\n\n3\n\n0\n\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSalyg1nIIgraSMEX"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_size_vec(n);

    let mut mex = n;
    for i in 0..n {
        if s[i] != i {
            mex = i;
            break;
        }
    }
    out_line!(mex);
    loop {
        let y = input.read_int();
        if y == -1 {
            break;
        }
        out_line!(y);
    }
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
    // input.skip_whitespace();
    // !input.peek().is_some()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
