//{"name":"Interesting Subarray","group":"CodeChef - SEP221A","url":"https://www.codechef.com/SEP221A/problems/SUBARRY","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n2 2\n3\n5 0 9\n","output":"4 4\n0 81\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"InterestingSubarray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    let mut max_positive = None;
    let mut min_negative = None;
    let mut closest_to_zero = i64::MAX;
    for i in a {
        if i > 0 {
            max_positive.maxim(i);
        } else if i < 0 {
            min_negative.minim(i);
        }
        closest_to_zero.minim(i.abs());
    }

    let (min, max) = if let Some(pos) = max_positive {
        if let Some(neg) = min_negative {
            (pos * neg, (pos * pos).max(neg * neg))
        } else {
            (closest_to_zero * closest_to_zero, pos * pos)
        }
    } else {
        if let Some(neg) = min_negative {
            (closest_to_zero * closest_to_zero, neg * neg)
        } else {
            (0, 0)
        }
    };
    out_line!(min, max);
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
