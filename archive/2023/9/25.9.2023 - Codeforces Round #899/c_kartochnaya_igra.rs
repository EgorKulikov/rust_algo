//{"name":"C. Карточная игра","group":"Codeforces - Codeforces Round 899 (Div. 2)","url":"https://codeforces.com/contest/1882/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n-4 1 -3 5\n4\n1 -2 3 -4\n3\n-1 3 -5\n1\n-1\n","output":"5\n4\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKartochnayaIgra"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut ans = 0;
    for &i in a.iter().skip(2) {
        ans += i.max(0);
    }
    if a[0] >= 0 {
        ans += a[0] + if n > 1 { a[1].max(0) } else { 0 };
    } else if n > 1 && a[0] + a[1] >= 0 {
        ans += a[0] + a[1];
    }
    out_line!(ans);
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
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
