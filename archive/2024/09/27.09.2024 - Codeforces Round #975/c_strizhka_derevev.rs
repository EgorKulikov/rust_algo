//{"name":"C. Стрижка деревьев","group":"Codeforces - Codeforces Round 975 (Div. 1)","url":"https://codeforces.com/contest/2018/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n7\n1 2\n1 3\n2 4\n2 5\n4 6\n4 7\n7\n1 2\n1 3\n1 4\n2 5\n3 6\n5 7\n15\n12 9\n1 6\n6 14\n9 11\n8 7\n3 5\n13 5\n6 10\n13 15\n13 6\n14 12\n7 2\n8 1\n1 4\n","output":"2\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CStrizhkaDerevev"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let lca = graph.lca();
    let mut children = Vec::new();
    let mut at_height = vec![Vec::new(); (0..n).map(|v| lca.level(v)).max().unwrap() + 1];
    let mut qty = vec![0usize; at_height.len()];
    for i in 0..n {
        qty[lca.level(i)] += 1;
        let mut num_children = 0;
        for e in &graph[i] {
            if lca.level(e.to()) > lca.level(i) {
                num_children += 1;
            }
        }
        if num_children == 0 {
            at_height[lca.level(i)].push(i);
        }
        children.push(num_children);
    }
    children[0] = n + 1;
    let qty = qty.partial_sums();
    let mut ans = None;
    let mut deleted = 0;
    let mut queue = Vec::new();
    for i in at_height.indices() {
        ans.minim(deleted + qty.backward()[0] - qty[i + 1]);
        for &j in &at_height[i] {
            queue.push(j);
        }
        while let Some(v) = queue.pop() {
            deleted += 1;
            let parent = lca.parent(v).unwrap();
            children[parent] -= 1;
            if children[parent] == 0 {
                queue.push(parent);
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
