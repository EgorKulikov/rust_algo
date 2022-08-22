//{"name":"Subtract 12 Operation","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/SUB12OP","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n2 4\n3\n1 1 1\n6\n-4 2 -4 2 -4 2\n1\n-100000000\n","output":"0\n2\n15\n100000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Subtract12Operation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    for i in (1..n).rev() {
        if a[i] > 1 {
            let by = a[i] / 2;
            a[i] -= 2 * by;
            a[i - 1] -= by;
        }
        // if a[i] == 1 && a[i - 1] > 0 && a[i - 1] % 2 == 1 {
        //     a[i] -= 2;
        //     a[i - 1] -= 1;
        // }
    }
    for i in 1..n {
        if a[i - 1] > 0 && a[i] == 1 {
            a[i - 1] -= 1;
            a[i] -= 2;
        }
    }
    out_line!(a.into_iter().map(i64::abs).sum::<i64>());
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
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
}
//END MAIN
