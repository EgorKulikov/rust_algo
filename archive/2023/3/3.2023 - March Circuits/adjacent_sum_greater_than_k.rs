//{"name":"Adjacent Sum Greater than K","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/adjacent-sum-greater-than-k-f41e3ec4/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 5\n4 6\n","output":"1 4 2 3 5 6\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AdjacentSumGreaterThanK"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size().max(2);

    if k > n + 1 {
        out_line!(-1);
        return;
    }
    let mut ans = Vec::with_capacity(n);
    for i in 1..=k / 2 {
        ans.push(i);
        if i != k - i {
            ans.push(k - i);
        }
    }
    for i in k..=n {
        ans.push(i);
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
