//{"name":"Distinct Values","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/DIST_VALS","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 1\n3\n4 2 1\n4\n8 1 7 2\n5\n6 9 4 2 1\n","output":"1\n2\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DistinctValues"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    let mut set = HashSet::new();
    for _ in 0..2 {
        let mut s = Vec::new();
        for &i in &a {
            while let Some(&x) = s.last() {
                if x <= i {
                    set.insert(i - x);
                    s.pop();
                } else {
                    break;
                }
            }
            s.push(i);
        }
        a.reverse();
    }
    out_line!(set.len());
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
