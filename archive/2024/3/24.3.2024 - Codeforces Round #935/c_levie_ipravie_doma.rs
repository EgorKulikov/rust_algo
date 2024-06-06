//{"name":"C. Левые и правые дома","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n101\n6\n010111\n6\n011001\n3\n000\n3\n110\n3\n001\n4\n1100\n","output":"2\n3\n2\n3\n0\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLevieIPravieDoma"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_str();

    let mut left_left = 0;
    let mut left_right = 0;
    let mut right_left = a.iter().count_eq(&b'0');
    let mut right_right = n - right_left;

    let mut ans = if right_right >= right_left {
        Some((n, 0))
    } else {
        None
    };
    for i in 0..n {
        if a[i] == b'0' {
            right_left -= 1;
            left_left += 1;
        } else {
            right_right -= 1;
            left_right += 1;
        }
        if left_left >= left_right && right_right >= right_left {
            ans.minim((n.abs_diff(2 * (i + 1)), i + 1));
        }
    }
    out.print_line(ans.unwrap().1);
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
