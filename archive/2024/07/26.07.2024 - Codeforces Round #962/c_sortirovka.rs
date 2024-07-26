//{"name":"C. Сортировка","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/C","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5 3\nabcde\nedcba\n1 5\n1 4\n3 3\n4 2\nzzde\nazbe\n1 3\n1 4\n6 3\nuwuwuw\nwuwuwu\n2 4\n1 3\n1 6\n","output":"0\n1\n0\n2\n2\n1\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSortirovka"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let _n = input.read_size();
    let q = input.read_size();
    let a = input.read_str();
    let b = input.read_str();

    fn cum_qty(s: &Str) -> Vec<Vec<usize>> {
        let mut qty = vec![vec![0; 26]];
        for i in s.indices() {
            let c = s[i] as usize - 'a' as usize;
            qty.push(qty[i].clone());
            qty[i + 1][c] += 1;
        }
        qty
    }

    let a_qty = cum_qty(&a);
    let b_qty = cum_qty(&b);

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;
        let mut ans = 0;
        for i in 0..26 {
            let a_qty = a_qty[r + 1][i] - a_qty[l][i];
            let b_qty = b_qty[r + 1][i] - b_qty[l][i];
            if a_qty >= b_qty {
                ans += a_qty - b_qty;
            }
        }
        out.print_line(ans);
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
}
//END MAIN
