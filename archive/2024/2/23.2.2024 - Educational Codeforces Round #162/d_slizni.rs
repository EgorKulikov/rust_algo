//{"name":"D. Слизни","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n3 2 4 2\n3\n1 2 3\n5\n2 2 3 1 1\n7\n4 2 3 6 1 1 8\n","output":"2 1 2 1\n1 1 -1\n2 1 -1 1 2\n2 1 1 3 1 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlizni"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let s = a.partial_sums();
    let mut next_diff = Vec::with_capacity(n);
    next_diff.push(n);
    for i in (0..n - 1).rev() {
        if a[i] != a[i + 1] {
            next_diff.push(i + 1);
        } else {
            next_diff.push(next_diff.last().copied().unwrap());
        }
    }
    next_diff.reverse();
    let mut prev_diff = Vec::with_capacity(n);
    prev_diff.push(0);
    for i in 1..n {
        if a[i] != a[i - 1] {
            prev_diff.push(i);
        } else {
            prev_diff.push(prev_diff.last().copied().unwrap());
        }
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if i != 0 && a[i] < a[i - 1] || i != n - 1 && a[i] < a[i + 1] {
            ans.push(1);
            continue;
        }
        let mut cur = None;
        if i != n - 1 && next_diff[i + 1] != n && s[n] - s[i + 1] > a[i] {
            let mut left = next_diff[i + 1] + 1;
            let mut right = n;
            while left < right {
                let mid = (left + right) / 2;
                if s[mid] - s[i + 1] > a[i] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            cur.minim(left - i - 1);
        }
        if i != 0 && prev_diff[i - 1] != 0 && s[i] > a[i] {
            let mut left = 0;
            let mut right = prev_diff[i - 1] - 1;
            while left < right {
                let mid = (left + right + 1) / 2;
                if s[i] - s[mid] > a[i] {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            cur.minim(i - left);
        }
        if let Some(x) = cur {
            ans.push(x as isize);
        } else {
            ans.push(-1);
        }
    }
    out.print_line(ans);
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
