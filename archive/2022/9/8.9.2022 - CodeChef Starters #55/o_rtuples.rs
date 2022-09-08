//{"name":"OR Tuples","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/ORTUPLES","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n10 12 14\n0 5 5\n0 0 7\n","output":"4\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ORTuples"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let tuple = input.read_unsigned_vec(3);

    let mut ans = 1i64;
    for i in 0..20 {
        let mut cnt = 0;
        for j in 0..3 {
            if tuple[j].is_set(i) {
                cnt += 1;
            }
        }
        match cnt {
            1 => {
                ans = 0;
                break;
            }
            3 => {
                ans *= 4;
            }
            _ => {}
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
}
//END MAIN
