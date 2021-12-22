//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/array-maximum-01fc2734/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n3\n","output":"2\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::primes;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    out_line!(primes::<usize>(n + 1).len());
}

#[test]
fn stress() {
    let mut num_primes = 0usize;
    for i in 2..10000 {
        let mut add = 1;
        for j in 2..i {
            if i % j == 0 {
                add = 0;
                break;
            }
        }
        num_primes += add;
        assert_eq!(primes::<usize>(i + 1).len(), num_primes);
    }
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
