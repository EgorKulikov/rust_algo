//{"name":"#4 - Putovanje","group":"DMOJ - COCI '23 Contest 4","url":"https://dmoj.ca/problem/coci23c4p4","interactive":false,"timeLimit":2000,"tests":[{"input":"7 6\n1 2\n1 3\n3 4\n3 5\n3 6\n5 7\n2 -1 -1 -1 -1 -1 3\n","output":"2\n4 6\n"},{"input":"6 6\n1 2\n2 3\n3 4\n4 5\n5 6\n6 1\n2 -1 -1 1 -1 -1\n","output":"2\n3 5\n"},{"input":"4 3\n1 2\n2 3\n3 4\n1 -1 -1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Putovanje"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();
    let d = input.read_vec::<isize>(n);

    let graph = Graph::from_biedges(n, &edges);
    let mut ans = BitSet::new(n);
    ans.fill(true);
    for i in 0..n {
        if d[i] == -1 {
            continue;
        }
        let d = d[i] as u32;
        let dist = graph.edge_distances(i);
        for j in 0..n {
            if dist[j] != d {
                ans.unset(j);
            }
        }
    }
    let ans = ans.iter().collect_vec().inc();
    out.print_line(ans.len());
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
