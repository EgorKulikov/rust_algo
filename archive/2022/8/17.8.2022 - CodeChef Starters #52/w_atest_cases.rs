//{"name":"WA Test Cases","group":"CodeChef - START52A","url":"https://www.codechef.com/START52A/problems-old/WATESTCASES","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n5 10 3\n000\n3\n5 10 3\n001\n3\n5 5 3\n001\n3\n5 5 3\n101\n5\n10 100 100 10 10\n00001\n","output":"3\n5\n5\n5\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WATestCases"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_int_vec(n);
    let v = input.read_vec::<char>(n);

    out_line!(
        s.into_iter()
            .zip(v)
            .filter(|&(_, c)| c == '0')
            .min()
            .unwrap()
            .0
    );
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
