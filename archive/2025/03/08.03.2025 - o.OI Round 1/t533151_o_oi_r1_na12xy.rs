//{"name":"T533151 「o.OI R1」na12xy","group":"Luogu","url":"https://www.luogu.com.cn/problem/T533151?contestId=224200","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2\n1 3\n","output":"5\n1 1 1\n1 2 2\n2 1 2\n1 12 3\n2 12 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n == 1 {
        out.print_line(0);
        return;
    }
    let graph = Graph::with_biedges(n, &edges);
    let mut start = 0;
    loop {
        let mut size = vec![1; n];
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                size[vert] += f.call(e.to(), vert);
            }
            size[vert]
        });
        dfs.call(start, start);
        let mut ans = Vec::new();
        let mut important = 0;
        ans.push((1, 1, start + 1));
        let mut new_prev = 0;
        let mut new_start = 0;
        let mut dfs = RecursiveFunction3::new(|dfs, vert: usize, prev: usize, id: usize| -> bool {
            let mut children = Vec::new();
            for e in &graph[vert] {
                if e.to() != prev {
                    children.push(e.to());
                }
            }
            children.sort_by_key(|&x| Reverse(size[x]));
            important.set_bit(id);
            for i in 1..children.len() {
                let mut free = 0;
                while important.is_set(free) {
                    free += 1;
                }
                if free == 12 {
                    new_start = children[i];
                    new_prev = vert;
                    return false;
                }
                ans.push((1, free + 1, children[i] + 1));
                ans.push((2, id + 1, free + 1));
                if !dfs.call(children[i], vert, free) {
                    return false;
                }
            }
            if !children.is_empty() {
                let mut free = 0;
                while important.is_set(free) {
                    free += 1;
                }
                if free == 12 {
                    new_start = children[0];
                    new_prev = vert;
                    return false;
                }
                important.unset_bit(id);
                ans.push((1, free + 1, children[0] + 1));
                ans.push((2, id + 1, free + 1));
                if !dfs.call(children[0], vert, free) {
                    return false;
                }
            }
            important.unset_bit(id);
            true
        });
        if dfs.call(start, start, 0) {
            out.print_line(ans.len());
            out.print_per_line(&ans);
            return;
        }
        let mut vert = start;
        let mut prev = start;
        loop {
            let mut children = Vec::new();
            for e in &graph[vert] {
                if e.to() != prev {
                    children.push(e.to());
                }
            }
            children.sort_by_key(|&x| Reverse(size[x]));
            if children.is_empty() {
                break;
            }
            (vert, prev) = (children[0], vert);
        }
        start = vert;
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
