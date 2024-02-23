//{"name":"A. Перемещение фишек","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n8\n0 1 1 1 0 1 1 0\n6\n0 1 0 0 0 0\n6\n1 1 1 1 1 1\n5\n1 0 1 0 1\n9\n0 1 1 0 0 0 1 1 0\n","output":"1\n0\n0\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APeremeshchenieFishek"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    let mut ans = 0;
    let qty = a.iter().count_eq(&&1);
    loop {
        let left = a.iter().find_eq(&1).unwrap();
        let right = n - 1 - a.iter().rev().find_eq(&1).unwrap();
        if right - left + 1 == qty {
            break;
        }
        let rightmost_zero = right - a.iter().take(right + 1).rev().find_eq(&0).unwrap();
        ans += 1;
        a[right] = 0;
        a[rightmost_zero] = 1;
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
