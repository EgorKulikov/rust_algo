//{"name":"C. Ближайшие города","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n5\n0 8 12 15 20\n5\n1 4\n1 5\n3 4\n3 2\n5 1\n","output":"3\n8\n1\n4\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBlizhaishieGoroda"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut right = Vec::with_capacity(n - 1);
    let mut left = Vec::with_capacity(n - 1);
    for i in 1..n {
        let left_good = i == 1 || a[i - 1] - a[i - 2] > a[i] - a[i - 1];
        let right_good = i == n - 1 || a[i + 1] - a[i] > a[i] - a[i - 1];
        if left_good {
            left.push(1);
        } else {
            left.push(a[i] - a[i - 1])
        }
        if right_good {
            right.push(1);
        } else {
            right.push(a[i] - a[i - 1])
        }
    }

    let left = left.partial_sums();
    let right = right.partial_sums();

    let k = input.read_size();
    for _ in 0..k {
        let x = input.read_size() - 1;
        let y = input.read_size() - 1;
        if x < y {
            out.print_line(left[y] - left[x]);
        } else {
            out.print_line(right[x] - right[y]);
        }
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
    let test_type = TestType::MultiNumber;
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
    //    tester::stress_test();
}
//END MAIN
