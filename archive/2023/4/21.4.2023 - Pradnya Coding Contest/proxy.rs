//{"name":"Proxy","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/PROXY2023","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n8\n0 1 1 0 1 0 0 1\n0 0 1 0 0 1 0 1\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Proxy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let present = input.read_int_vec(n);
    let check = input.read_int_vec(n);

    let mut ans = 0;
    let mut can = 0;
    for (p, c) in present.into_iter().zip(check.into_iter()) {
        if p == 1 {
            ans += 1;
            can += 1;
        } else if c == 0 {
            if can > 0 {
                ans += 1;
                can -= 1;
            }
        }
    }
    out_line!(ans);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
