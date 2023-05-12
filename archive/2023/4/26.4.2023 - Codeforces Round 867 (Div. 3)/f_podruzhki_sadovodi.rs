//{"name":"F. Подружки-садоводы","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2 3\n2 1\n3 1\n5 4 1\n2 1\n4 2\n5 4\n3 4\n6 5 3\n4 1\n6 1\n2 6\n5 1\n3 2\n10 6 4\n1 3\n1 9\n9 7\n7 6\n6 4\n9 2\n2 8\n8 5\n5 10\n","output":"2\n12\n17\n32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPodruzhkiSadovodi"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_long();
    let c = input.read_long();
    let edges = input.read_size_pair_vec(n - 1).dec_by_one();

    let graph = Graph::from_biedges(n, &edges);
    let mut dist_to_leaf = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            dist_to_leaf[vert].maxim(f.call(e.to(), vert) + k);
        }
        dist_to_leaf[vert]
    });
    dfs.call(0, n);
    let mut dfs2 = RecursiveFunction3::new(|f, vert: usize, prev: usize, best_up: i64| {
        let mut ans = dist_to_leaf[vert].max(best_up);
        let mut best_down = 0;
        let mut second_best = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let to = e.to();
            if best_down < dist_to_leaf[to] + k {
                second_best = best_down;
                best_down = dist_to_leaf[to] + k;
            } else {
                second_best.maxim(dist_to_leaf[to] + k);
            }
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let to = e.to();
            let cur_best_up = if best_down == dist_to_leaf[to] + k {
                second_best
            } else {
                best_down
            }
            .max(best_up)
                + k;
            ans.maxim(f.call(e.to(), vert, cur_best_up) - c);
        }
        ans
    });
    out_line!(dfs2.call(0, n, 0));
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
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
