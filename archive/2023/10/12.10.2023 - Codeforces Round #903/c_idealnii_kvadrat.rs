//{"name":"C. Идеальный квадрат","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\nabba\nbcbb\nbccb\nabba\n2\nab\nba\n6\ncodefo\nrcesco\ndeforc\nescode\nforces\ncodefo\n4\nbaaa\nabba\nbaba\nbaab\n4\nbbaa\nabba\naaba\nabba\n","output":"1\n2\n181\n5\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIdealniiKvadrat"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mat = input.read_char_table(n, n);

    let mut ans = 0;
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            let mut v = Vec::with_capacity(4);
            v.push(mat[(i, j)] as i32);
            v.push(mat[(n - j - 1, i)] as i32);
            v.push(mat[(n - i - 1, n - j - 1)] as i32);
            v.push(mat[(j, n - i - 1)] as i32);
            let max = *v.iter().max().unwrap();
            for k in v {
                ans += max - k;
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
