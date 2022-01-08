//{"name":"Non-empty subsets","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/minor-4-41918cb8/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n1 3 5 7\n3\n2 6 14\n","output":"1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NonEmptySubsets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    out_line!(a.into_iter().min());
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
