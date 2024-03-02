//{"name":"B. Пинбол","group":"Codeforces - Codeforces Round 930 (Div. 1)","url":"https://codeforces.com/contest/1936/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n><<\n4\n<<<<\n6\n<><<<>\n","output":"3 6 5\n1 2 3 4\n1 4 7 10 8 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPinbol"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut right = Vec::new();
    for i in 0..n {
        if s[i] == b'<' {
            right.push(i);
        }
    }
    right.reverse();
    let mut right_sum = right.partial_sums();
    right.reverse();
    right_sum.reverse();
    let mut left = Vec::new();
    let mut left_sum = vec![0];
    let mut right_pos = 0;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if s[i] == b'>' {
            if right.len() - right_pos > left.len() {
                ans.push(
                    2 * (right_sum[right_pos] - right_sum[right_pos + left.len() + 1])
                        - 2 * left_sum[left.len()]
                        - i
                        + 1,
                );
            } else {
                ans.push(
                    n + 2 * (right_sum[right_pos] - right_sum[right.len()])
                        - 2 * (left_sum[left.len()]
                            - left_sum[left.len() - (right.len() - right_pos)])
                        - i,
                );
            }
            left_sum.push(left_sum[left.len()] + i);
            left.push(i);
        } else {
            right_pos += 1;
            if right.len() - right_pos >= left.len() {
                ans.push(
                    2 * (right_sum[right_pos] - right_sum[right_pos + left.len()]) + i
                        - 2 * left_sum[left.len()]
                        + 1,
                );
            } else {
                ans.push(
                    n + i + 2 * (right_sum[right_pos] - right_sum[right.len()])
                        - 2 * (left_sum[left.len()]
                            - left_sum[left.len() - (right.len() - right_pos) - 1]),
                );
            }
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
