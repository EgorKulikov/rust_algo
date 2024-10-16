//{"name":"Keep Magnets Separated","group":"CodeChef - START156A","url":"https://www.codechef.com/START156A/problems/MAGNET","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n1 2\n4\n1 2\n2 3\n3 4\n","output":"-1\n3 4 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KeepMagnetsSeparated"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    if (0..n).any(|i| graph[i].len() == n - 1) {
        out.print_line(-1);
        return;
    }

    let mut center = None;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut size = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            size += f.call(e.to(), vert);
        }
        if 2 * size > n && center.is_none() {
            center = Some(vert);
        }
        size
    });
    dfs.call(0, n);
    if center.is_none() {
        loop {}
    }
    let center = center.unwrap();
    let mut components = Vec::new();
    let mut big = 0;
    let mut big_at = 0;
    for e in &graph[center] {
        let mut cur = Vec::new();
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            cur.push(vert);
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
            }
        });
        dfs.call(e.to(), center);
        assert!(!cur.is_empty());
        if cur.len() > 1 {
            big += 1;
            big_at = components.len();
        }
        cur.reverse();
        components.push(cur);
    }
    let mut ans = vec![center];
    let mut heap = BinaryHeap::new();
    let mut last: Option<usize> = None;
    for i in components.indices() {
        if big == 1 && i == big_at {
            last = Some(i);
        } else {
            heap.push((components[i].len(), i));
        }
    }
    for _ in 0..n - 1 {
        if heap.is_empty() {
            loop {}
        }
        let (_, next) = heap.pop().unwrap();
        if components[next].is_empty() {
            loop {}
        }
        ans.push(components[next].pop().unwrap());
        if let Some(last) = last {
            if components[last].is_empty() {
                panic!();
            }
            heap.push((components[last].len(), last));
        }
        last = None;
        if !components[next].is_empty() {
            last = Some(next);
            if heap.is_empty() {
                heap.push((components[next].len(), next));
                last = None;
            }
        }
    }
    out.print_line(ans.inc());
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
