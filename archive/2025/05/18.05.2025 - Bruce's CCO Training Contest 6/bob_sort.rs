//{"name":"Bob Sort","group":"DMOJ","url":"https://dmoj.ca/problem/oly22practice37","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5\n3 2 3 1 1\n","output":"1\n5\n1 4 2 3 5\n"},{"input":"6 9\n6 5 4 3 2 1\n","output":"2\n6\n1 6 2 5 3 4\n3\n3 2 1\n"},{"input":"4 3\n2 1 4 3\n","output":"-1\n"},{"input":"2 0\n2 2\n","output":"0\n"},{"input":"6 8\n6 5 4 3 2 1\n","output":"3\n2\n3 4\n4\n1 6 2 5\n2\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;
use std::mem::take;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let Compressed { arrs: [b], .. } = compress([&a]);
    let a = b.clone().sorted();
    let mut graph = Graph::new(n);
    // let mut dsu = DSU::new(n);
    let mut total = 0;
    for i in 0..n {
        if a[i] != b[i] {
            // dsu.union(a[i], b[i]);
            graph.add_edge(Edge::with_payload(a[i], b[i], i));
            total += 1;
        }
    }
    if total == 0 {
        out.print_line(0);
        return;
    }
    if total > m {
        out.print_line(-1);
        return;
    }
    let mut id = vec![0; n];
    let mut cycle = vec![Vec::new(); n];
    let mut order = (0..n).collect::<VecDeque<_>>();
    while let Some(i) = order.pop_front() {
        let mut at = i;
        while id[at] != graph[at].len() {
            order.push_front(at);
            cycle[i].push(*graph[at][id[at]].payload());
            id[at] += 1;
            at = graph[at][id[at] - 1].to();
        }
        assert_eq!(at, i);
    }
    let mut big_cycles = Vec::new();
    for i in 0..n {
        if !cycle[i].is_empty() {
            let mut cur = Vec::new();
            let mut rec = RecursiveFunction::new(|rec, pos: usize| {
                let c = take(&mut cycle[pos]);
                let mut at = pos;
                for i in c {
                    if !cycle[at].is_empty() {
                        rec.call(at);
                    }
                    cur.push(i);
                    at = b[i];
                }
            });
            rec.call(i);
            big_cycles.push(cur.inc());
        }
    }
    let bonus = (m - total).min(big_cycles.len());
    let mut ans = Vec::new();
    if bonus >= 3 {
        let mut rev = Vec::new();
        let mut big = Vec::new();
        for i in 0..bonus {
            big.extend_from_slice(&big_cycles[i]);
            rev.push(big_cycles[i][0]);
        }
        rev.reverse();
        ans.push(big);
        ans.push(rev);
        ans.extend_from_slice(&big_cycles[bonus..]);
    } else {
        ans = big_cycles;
    }
    out.print_line(ans.len());
    for i in ans {
        out.print_line(i.len());
        out.print_line(i);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
