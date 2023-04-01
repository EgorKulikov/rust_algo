//{"name":"Equal Diverse Teams","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/equal-diverse-teams-cbdb8fe2/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 2\n1 4 4 6 2 1\n4 2\n1 1 1 1\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EqualDiverseTeams"}}}

use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec_by_one();

    let q = a.qty_bound(n);
    let single = q.iter().filter(|&&v| v == 1).count();
    let multi = q.iter().filter(|&&v| v > 1).count();
    out_line!(single + multi <= 2 * k && single + 2 * multi >= 2 * k);
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
