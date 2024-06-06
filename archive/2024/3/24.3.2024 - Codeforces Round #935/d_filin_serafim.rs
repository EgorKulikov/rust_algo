//{"name":"D. Филин Серафим","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 2\n7 3 6 9\n4 3 8 5\n6 2\n6 9 7 1 8 3\n5 8 8 1 4 1\n7 7\n7 2 9 2 6 5 9\n9 1 10 7 1 4 9\n2 1\n2 3\n1 1\n","output":"14\n22\n9\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFilinSerafim"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut ans = a[m - 1];
    let mut cur = b[m - 1];
    for i in (0..m - 1).rev() {
        ans.minim(cur + a[i]);
        cur += b[i];
    }
    for i in m..n {
        ans += a[i].min(b[i]);
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
