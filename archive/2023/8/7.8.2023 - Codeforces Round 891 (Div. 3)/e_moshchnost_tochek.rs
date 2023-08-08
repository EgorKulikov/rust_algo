//{"name":"E. Мощность точек","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 4 3\n5\n1 2 5 7 1\n4\n1 10 100 1000\n","output":"8 7 6\n16 15 18 24 16\n1111 1093 1093 2893\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMoshchnostTochek"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| x[i]);
    let mut less = 0;
    let mut more = n;
    let mut less_sum = 0;
    let mut more_sum = x.iter().sum::<usize>();
    let mut ans = vec![0; n];
    for i in order {
        more -= 1;
        more_sum -= x[i];
        ans[i] = n + less * x[i] - less_sum + more_sum - more * x[i];
        less += 1;
        less_sum += x[i];
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
