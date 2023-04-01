//{"name":"Largest Number","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/largest-number-7-eee0b7c3/","interactive":false,"timeLimit":1000,"tests":[{"input":"44312\n2\n","output":"443\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LargestNumber"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: Str = input.read();
    let mut k = input.read_size();

    let mut next = Arr2d::new(n.len() + 1, 10, n.len());
    for i in (0..n.len()).rev() {
        for j in 0..10 {
            next[(i, j)] = next[(i + 1, j)];
        }
        next[(i, (n[i] - b'0').into_usize())] = i;
    }
    let mut at = 0;
    for _ in 0..n.len() - k {
        for j in (0..10).rev() {
            if next[(at, j)] <= at + k {
                out!(j);
                k -= next[(at, j)] - at;
                at = next[(at, j)] + 1;
                break;
            }
        }
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
