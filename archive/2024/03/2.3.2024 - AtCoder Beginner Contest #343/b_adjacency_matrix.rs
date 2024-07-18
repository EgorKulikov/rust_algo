//{"name":"B - Adjacency Matrix","group":"AtCoder - AtCoder Beginner Contest 343","url":"https://atcoder.jp/contests/abc343/tasks/abc343_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n0 1 1 0\n1 0 0 1\n1 0 0 0\n0 1 0 0\n","output":"2 3\n1 4\n1\n2\n"},{"input":"2\n0 0\n0 0\n","output":"\n"},{"input":"5\n0 1 0 1 1\n1 0 0 1 0\n0 0 0 0 1\n1 1 0 0 1\n1 0 1 1 0\n","output":"2 4 5\n1 4\n5\n1 2 5\n1 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAdjacencyMatrix"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_table(n, n);

    let mut ans = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if a[(i, j)] == 1 {
                ans[i].push(j + 1);
            }
        }
    }
    out.print_per_line(&ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
