//{"name":"E. Clique Partition","group":"Codeforces - Educational Codeforces Round 163 (Rated for Div. 2)","url":"https://codeforces.com/contest/1948/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2 3\n5 4\n8 16\n","output":"2 1\n1\n1 1\n3 1 5 2 4\n2\n1 1 2 1 2\n1 2 3 4 5 6 7 8\n1\n1 1 1 1 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECliquePartition"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut ans = (1..=n).collect_vec();
    for i in (0..n).step_by(k) {
        ans[i..(i + k / 2).min(n)].reverse();
        ans[(i + k / 2).min(n)..(i + k).min(n)].reverse();
    }
    let c = (0..n).map(|i| (i / k) + 1).collect_vec();
    out.print_line(ans);
    out.print_line(n.div_ceil(k));
    out.print_line(c);
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
