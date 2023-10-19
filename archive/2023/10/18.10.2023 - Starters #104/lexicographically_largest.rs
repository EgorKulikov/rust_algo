//{"name":"Lexicographically Largest ","group":"CodeChef - START104A","url":"https://www.codechef.com/START104A/problems/LEXILARGEST","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\n1\n2 2\n2 1\n4 3\n2 2 2 2\n4 5\n2 2 2 2\n","output":"1\n2 1\n2 2 2 2\n2 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LexicographicallyLargest"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_int();
    let a = input.read_int_vec(n);

    let mut ans = Vec::with_capacity(n);
    ans.push(a[0]);
    for i in 1..n {
        let mut cur = (m / a[i]) * a[i];
        while gcd(a[i - 1], cur) != a[i] {
            cur -= a[i];
        }
        ans.push(cur);
    }
    out.print_line(ans);
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
