//{"name":"F - Breakdown","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_f","interactive":false,"timeLimit":2000,"tests":[{"input":"6 6\n1 2\n2 3\n3 1\n3 4\n1 5\n5 6\n9 2 3 1 4 4\n1 0 0 0 0 1\n","output":"5\n"},{"input":"2 1\n1 2\n1 2\n0 0\n","output":"0\n"},{"input":"10 20\n4 8\n1 10\n1 7\n5 9\n9 10\n8 10\n7 5\n1 4\n7 3\n8 7\n2 8\n5 8\n4 2\n5 1\n7 2\n8 3\n3 4\n8 9\n7 10\n2 3\n25 5 1 1 16 5 98 3 21 1\n35 39 32 11 35 37 14 29 36 1\n","output":"1380\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBreakdown"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();
    let w = input.read_size_vec(n);
    let a = input.read_long_vec(n);

    let graph = Graph::from_biedges(n, &edges);
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| w[i]);
    let mut ans = 0;
    let mut res = vec![0; n];
    for i in order {
        let mut cur = vec![1; w[i]];
        for e in &graph[i] {
            let to = e.to();
            if w[to] < w[i] {
                for j in (0..w[i] - w[to]).rev() {
                    let cand = cur[j] + res[to];
                    cur[j + w[to]].maxim(cand);
                }
            }
        }
        res[i] = cur[w[i] - 1];
        ans += res[i] * a[i];
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
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
