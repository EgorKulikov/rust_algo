//{"name":"D - K-th Nearest","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_d","interactive":false,"timeLimit":3000,"tests":[{"input":"4 3\n-3 -1 5 6\n-2 3\n2 1\n10 4\n","output":"7\n3\n13\n"},{"input":"2 2\n0 0\n0 1\n0 2\n","output":"0\n0\n"},{"input":"10 5\n-84 -60 -41 -100 8 -8 -52 -62 -61 -76\n-52 5\n14 4\n-2 6\n46 2\n26 7\n","output":"11\n66\n59\n54\n88\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKThNearest"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n).sorted();

    for _ in 0..q {
        let b = input.read_int();
        let k = input.read_size();
        let mut left = 0;
        let mut right = 200_000_000;
        while left < right {
            let mid = (left + right) / 2;
            let pts = a.upper_bound(&(b + mid)) - a.lower_bound(&(b - mid));
            if pts >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        out.print_line(left);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
