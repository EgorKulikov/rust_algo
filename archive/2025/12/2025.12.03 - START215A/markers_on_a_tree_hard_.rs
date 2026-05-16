//{"name":"Markers on a Tree (Hard)","group":"CodeChef - START215A","url":"https://www.codechef.com/START215A/problems/MARKTREEHD","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1\n3\n1 2\n5\n1 2 1 3\n7\n1 2 1 4 4 2\n6\n1 2 3 4 5\n","output":"0\n8\n96\n800\n296\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::{Callable2, Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::powers;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiWeightedEdge::new(p[i], i + 1, 1));
    }
    let d = graph.zero_one_distances_from(0);
    let mut path = vec![n - 1];
    let mut cur = n - 1;
    while cur != 0 {
        let prev = d[cur].unwrap().1;
        cur = prev;
        path.push(cur);
    }
    type Mod = ModIntF;
    let mut ans = Mod::zero();
    let pw = powers(Mod::new(2), n + 1);
    let mut sz = Vec::new();
    for i in path.indices() {
        let mut dfs =
            RecursiveFunction3::new(|f, vert: usize, prev1: usize, prev2: usize| -> usize {
                let mut res = 1;
                for e in &graph[vert] {
                    if e.to() == prev1 || e.to() == prev2 {
                        continue;
                    }
                    let call = f.call(e.to(), vert, vert);
                    ans += (pw[call] - 1) * pw[n - call] * 2;
                    res += call;
                }
                res
            });
        sz.push(dfs.call(
            path[i],
            path[i.saturating_sub(1)],
            path[(i + 1).min(path.len() - 1)],
        ));
    }
    let mut left = Memoization2d::new(sz.len(), sz.len(), |mem, pos, max| -> Mod {
        if pos == 0 {
            pw[sz[pos]]
        } else {
            let sum = mem.call(pos - 1, max);
            let mut cur = sum;
            if pos > max + 1 {
                cur -= mem.call(pos - max - 2, max);
            }
            sum + cur * (pw[sz[pos]] - 1)
        }
    });
    let mut right = Memoization2d::new(sz.len(), sz.len(), |mem, pos, max| -> Mod {
        if pos == sz.len() - 1 {
            pw[sz[pos]]
        } else {
            let sum = mem.call(pos + 1, max);
            let mut cur = sum;
            if pos + max + 2 < sz.len() {
                cur -= mem.call(pos + max + 2, max);
            }
            sum + cur * (pw[sz[pos]] - 1)
        }
    });
    let mut mult = pw[sz[0] + sz[Back(0)]];
    let mut total = Mod::zero();
    for i in 1..sz.len() - 1 {
        mult *= pw[sz[i]] - 1;
        for j in i..sz.len() - 1 {
            let max = j - i + 1;
            let mut l = left.call(i - 1, max - 1);
            if i != 1 {
                l -= left.call(i - 2, max - 1);
            }
            let mut r = right.call(j + 1, max);
            if j + 1 != sz.len() - 1 {
                r -= right.call(j + 2, max);
            }
            ans += l * r * (sz.len() - 2 - max) * 2;
            total += l * r;
        }
    }
    ans += mult * (sz.len() - 2) * 2;
    total += mult;
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
