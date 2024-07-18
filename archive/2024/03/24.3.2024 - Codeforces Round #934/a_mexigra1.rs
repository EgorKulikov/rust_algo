//{"name":"A. MEX игра - 1","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n0 0 1 1\n4\n0 1 2 3\n2\n1 1\n","output":"2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEXIgra1"}}}

use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let q = a.qty_bound(n + 1);
    let mut left = 0;
    let mut right = (n + 1) / 2;
    while left < right {
        let mid = (left + right + 1) / 2;
        let q = q[0..mid].to_vec().sorted();
        let mut ok = true;
        for (i, q) in q.into_iter().enumerate() {
            if i == 0 && q == 0 || i > 0 && q < 2 {
                ok = false;
                break;
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    out.print_line(left);
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
