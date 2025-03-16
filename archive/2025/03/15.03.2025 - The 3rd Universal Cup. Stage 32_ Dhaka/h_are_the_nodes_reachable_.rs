//{"name":"H. Are the nodes reachable?","group":"Universal Cup - The 3rd Universal Cup. Stage 32: Dhaka","url":"https://contest.ucup.ac/contest/1939/problem/10221","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n4 4\n1 2\n1 3\n1 4\n4 3\n2\n2 3\n2 4\n","output":"1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::splits::Split;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let order = graph.topological_sort().unwrap();
    let mut reachable_forward = vec![BitSet::new(n); n];
    for i in order.copy_rev() {
        reachable_forward[i].set(i);
        for e in &graph[i] {
            let (rfi, rfj) = reachable_forward.two_mut(i, e.to());
            *rfi |= &rfj;
        }
    }
    let set_size = n.upper_div(64);
    let first_forward = Arr2d::with_gen(n, set_size, |i, j| {
        if reachable_forward[i].raw(j) != 0 {
            reachable_forward[i].raw(j).lowest_bit() + j * 64
        } else {
            0
        }
    });
    let last_forward = Arr2d::with_gen(n, set_size, |i, j| {
        if reachable_forward[i].raw(j) != 0 {
            reachable_forward[i].raw(j).highest_bit() + j * 64
        } else {
            0
        }
    });
    let graph = Graph::new(n).do_with(|g| {
        for (u, v) in edges {
            g.add_edge(Edge::new(v, u));
        }
    });
    let order = graph.topological_sort().unwrap();
    let mut reachable_backward = vec![BitSet::new(n); n];
    for i in order.copy_rev() {
        reachable_backward[i].set(i);
        for e in &graph[i] {
            let (rbi, rbj) = reachable_backward.two_mut(i, e.to());
            *rbi |= &rbj;
        }
    }
    let first_backward = Arr2d::with_gen(n, set_size, |i, j| {
        if reachable_backward[i].raw(j) != 0 {
            reachable_backward[i].raw(j).lowest_bit() + j * 64
        } else {
            0
        }
    });
    let last_backward = Arr2d::with_gen(n, set_size, |i, j| {
        if reachable_backward[i].raw(j) != 0 {
            reachable_backward[i].raw(j).highest_bit() + j * 64
        } else {
            0
        }
    });
    let spread = Vec::with_gen_prefix(64, |i, s: &Vec<Vec<BitSet>>| {
        if i == 0 {
            reachable_forward.clone()
        } else {
            let mut cur = s[i - 1].clone();
            for j in 0..n {
                cur[j].shift_right_or(1);
                cur[j].shift_left_or(1);
            }
            cur
        }
    });

    let q = input.read_size();
    for _ in 0..q {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;

        if reachable_forward[u][v] {
            out.print_line(0);
        } else {
            let mut last_u = None;
            let mut last_v = None;
            let mut ans = usize::MAX;
            let mut need_other = false;
            for i in 0..set_size {
                let u_val = reachable_forward[u].raw(i);
                let v_val = reachable_backward[v].raw(i);
                if u_val != 0 && v_val != 0 {
                    need_other = true;
                    break;
                }
                if u_val != 0 {
                    if let Some(last_v) = last_v {
                        ans.minim(first_forward[(u, i)] - last_v);
                    }
                    last_u = Some(last_forward[(u, i)]);
                }
                if v_val != 0 {
                    if let Some(last_u) = last_u {
                        ans.minim(first_backward[(v, i)] - last_u);
                    }
                    last_v = Some(last_backward[(v, i)]);
                }
            }
            if need_other {
                ans.minim(64);
                for i in 0..set_size {
                    while (spread[ans - 1][u].raw(i) & reachable_backward[v].raw(i)) != 0 {
                        ans -= 1;
                    }
                }
            }
            out.print_line(ans);
        }
    }
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
