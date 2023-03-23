//{"name":"E. Две бочки","group":"Codeforces - Educational Codeforces Round 145 (Rated for Div. 2)","url":"https://codeforces.com/contest/1809/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 4\n-2 1 2\n","output":"0 0 0 0 0\n0 0 0 0 1\n0 0 1 1 2\n0 1 1 2 3\n1 1 2 3 4\n"},{"input":"3 9 5\n1 -2 2\n","output":"0 0 0 0 0 0\n0 0 0 0 0 1\n0 1 1 1 1 2\n1 2 2 2 2 3\n2 3 3 3 3 4\n3 4 4 4 4 5\n4 5 5 5 5 6\n5 6 6 6 6 7\n6 7 7 7 7 8\n7 7 7 7 8 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDveBochki"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int();
    let b = input.read_int();
    let v = input.read_int_vec(n);

    let mut ans = Arr2d::new((a + 1).into_usize(), (b + 1).into_usize(), 0);
    for i in 0..=(a + b) {
        let mut left = i - b.min(i);
        let mut left_qty = 1;
        let mut right = a.min(i);
        let min = left;
        let max = right;
        for &j in &v {
            left -= j;
            right -= j;
            if left < min {
                left_qty += min - left;
                left_qty.minim(max - min + 1);
                left = min;
            } else if left > max {
                left = max;
                left_qty = max - min + 1;
            }
            if right < min {
                right = min;
            } else if right > max {
                right = max;
            }
        }
        for j in min..=max {
            let cur = if left_qty > 0 {
                left_qty -= 1;
                left
            } else if left < right {
                left += 1;
                left
            } else {
                right
            };
            ans[(j.into_usize(), (i - j).into_usize())] = cur;
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
