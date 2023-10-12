//{"name":"E. Блоковая последовательность","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n7\n3 3 4 5 2 6 1\n4\n5 6 3 2\n6\n3 4 1 6 7 7\n3\n1 4 3\n5\n1 2 3 4 5\n5\n1 2 3 1 2\n5\n4 5 5 1 5\n","output":"0\n4\n1\n1\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBlokovayaPosledovatelnost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut mem = Memoization1d::new(n + 1, |f, i| -> usize {
        if i == n {
            0
        } else {
            let res = 1 + f.call(i + 1);
            if i + a[i] < n {
                res.min(f.call(i + a[i] + 1))
            } else {
                res
            }
        }
    });
    out.print_line(mem.call(0));
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
            for i in 0usize..t {
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
