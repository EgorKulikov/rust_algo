//{"name":"P5 - Road Redistribution","group":"DMOJ - Yet Another Contest 9","url":"https://dmoj.ca/problem/yac9p5","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2\n2 3\n3 4\n1 4\n2 1\n2 3\n2 4\n1 3\n","output":"2\n3 4 4 2\n1 4 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5RoadRedistribution"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_size_pair_vec(n).dec();
    let t = input.read_size_pair_vec(n).dec();

    let mut graph = vec![FxHashSet::default(); n];
    for (u, v) in s.copy_iter() {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    let mut stack = Vec::new();
    let mut cycle = Vec::new();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> bool {
        if let Some(pos) = stack.copy_find(vert) {
            cycle = stack[pos..].to_vec();
            return true;
        }
        stack.push(vert);
        for e in graph[vert].copy_iter() {
            if e == prev {
                continue;
            }
            if f.call(e, vert) {
                return true;
            }
        }
        stack.pop();
        false
    });
    assert!(dfs.call(0, n));
    let mut ans = Vec::new();
    while cycle.len() != n {
        for i in cycle.indices() {
            let u = cycle[i];
            if graph[u].len() > 2 {
                let mut to_add = 0;
                for j in graph[u].copy_iter() {
                    if !cycle.contains(&j) {
                        to_add = j;
                        break;
                    }
                }
                let next = cycle[(i + 1) % cycle.len()];
                ans.push((u + 1, next + 1, to_add + 1, next + 1));
                graph[u].remove(&next);
                graph[next].remove(&u);
                assert!(!graph[next].contains(&to_add));
                assert!(!graph[to_add].contains(&next));
                graph[next].insert(to_add);
                graph[to_add].insert(next);
                cycle.insert(i + 1, to_add);
                break;
            }
        }
    }

    let mut t_deg = vec![0; n];
    for (u, v) in t.copy_iter() {
        t_deg[u] += 1;
        t_deg[v] += 1;
    }
    while let Some(vert) = t_deg.copy_find(1) {
        let mut other = 0;
        for (u, v) in t.copy_iter() {
            if u == vert && t_deg[v] > 0 {
                other = v;
                break;
            }
            if v == vert && t_deg[u] > 0 {
                other = u;
                break;
            }
        }
        t_deg[vert] = 0;
        t_deg[other] -= 1;
        let pos = cycle.copy_find(vert).unwrap();
        let next = cycle[(pos + 1) % cycle.len()];
        let prev = cycle[(pos + cycle.len() - 1) % cycle.len()];
        if next == other {
            ans.push((vert + 1, prev + 1, next + 1, prev + 1));
            cycle.remove(pos);
            continue;
        }
        if prev == other {
            ans.push((vert + 1, next + 1, prev + 1, next + 1));
            cycle.remove(pos);
            continue;
        }
        ans.push((vert + 1, prev + 1, vert + 1, other + 1));
        ans.push((vert + 1, next + 1, prev + 1, next + 1));
        cycle.remove(pos);
    }
    let head = t_deg.copy_find(2).unwrap();
    let mut v1 = n;
    let mut v2 = n;
    for (u, v) in t.copy_iter() {
        if u == head && t_deg[v] == 2 {
            if v1 == n {
                v1 = v;
            } else {
                v2 = v;
            }
        }
        if v == head && t_deg[u] == 2 {
            if v1 == n {
                v1 = u;
            } else {
                v2 = u;
            }
        }
    }
    while cycle.len() > 3 {
        let mut pos = 0;
        while cycle[pos] == head || cycle[pos] == v1 || cycle[pos] == v2 {
            pos += 1;
        }
        let vert = cycle[pos];
        let next = cycle[(pos + 1) % cycle.len()];
        let prev = cycle[(pos + cycle.len() - 1) % cycle.len()];
        if next == head {
            ans.push((vert + 1, prev + 1, next + 1, prev + 1));
            cycle.remove(pos);
            continue;
        }
        if prev == head {
            ans.push((vert + 1, next + 1, prev + 1, next + 1));
            cycle.remove(pos);
            continue;
        }
        ans.push((vert + 1, prev + 1, vert + 1, head + 1));
        ans.push((vert + 1, next + 1, prev + 1, next + 1));
        cycle.remove(pos);
    }
    let mut prev = head;
    loop {
        let mut other = 0;
        for (u, v) in t.copy_iter() {
            if u == v1 && v != prev && t_deg[v] == 2 {
                other = v;
                break;
            }
            if v == v1 && u != prev && t_deg[u] == 2 {
                other = u;
                break;
            }
        }
        if other == v2 {
            break;
        }
        ans.push((v1 + 1, v2 + 1, v1 + 1, other + 1));
        ans.push((other + 1, head + 1, other + 1, v2 + 1));
        prev = v1;
        v1 = other;
        assert!(ans.len() <= 10 * n);
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
