//{"name":"A. Hey, Have You Seen My Kangaroo?","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9564","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3 6\nULDDRR\n010\n111\n010\n","output":"-1\n4\n2\n1\n0\n0\n0\n0\n0\n"},{"input":"3 3 6\nULDDRR\n010\n111\n011\n","output":"7\n4\n2\n1\n1\n0\n0\n0\n0\n"},{"input":"1 5 1\nR\n11111\n","output":"4\n3\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHeyHaveYouSeenMyKangaroo"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_str();
    let a = input.read_char_table(n, m);

    let mut graph = Graph::new(n * m);
    let mut is_root = BitSet::new(n * m);
    let mut x = vec![None; n * m];

    let mv = |r: usize, c: usize, cc: u8| -> (usize, usize) {
        match cc {
            b'U' => {
                if r > 0 && a[(r - 1, c)] == b'1' {
                    return (r - 1, c);
                }
            }
            b'D' => {
                if r + 1 < n && a[(r + 1, c)] == b'1' {
                    return (r + 1, c);
                }
            }
            b'L' => {
                if c > 0 && a[(r, c - 1)] == b'1' {
                    return (r, c - 1);
                }
            }
            b'R' => {
                if c + 1 < m && a[(r, c + 1)] == b'1' {
                    return (r, c + 1);
                }
            }
            _ => unreachable!(),
        }
        (r, c)
    };
    for i in 0..n {
        for j in 0..m {
            if a[(i, j)] == b'0' {
                continue;
            }
            let mut r = i;
            let mut c = j;
            for cc in s.iter() {
                (r, c) = mv(r, c, cc);
            }
            x[i * m + j] = Some(r * m + c);
        }
    }
    let mut done = vec![0; n * m];
    let mut root_shift = (0..n * m).collect_vec();
    for i in 0..n * m {
        if done[i] == 2 || x[i].is_none() {
            continue;
        }
        let mut cur = i;
        while done[cur] == 0 {
            done[cur] = 1;
            cur = x[cur].unwrap();
        }
        while done[cur] == 1 {
            is_root.set(cur);
            done[cur] = 2;
            root_shift[x[cur].unwrap()] = cur;
            cur = x[cur].unwrap();
        }
        cur = i;
        while done[cur] == 1 {
            done[cur] = 2;
            cur = x[cur].unwrap();
        }
    }
    for i in 0..n * m {
        if let Some(to) = x[i] {
            if !is_root[i] {
                graph.add_edge(Edge::new(root_shift[to], i));
            }
        }
    }
    let mut joins = Vec::new();
    let mut ans = Vec::with_capacity(n * m);
    for i in is_root.iter() {
        let mut rec = RecursiveFunction::new(|rec, vert: usize| -> usize {
            let mut lens = Vec::new();
            let mut pos = Vec::new();
            for e in &graph[vert] {
                lens.push(rec.call(e.to()) + 1);
                pos.push((e.to() / m, e.to() % m));
            }
            if is_root[vert] {
                lens.push(usize::MAX);
                pos.push((vert / m, vert % m));
            }
            let mut good = BitSet::new(pos.len());
            good.fill(true);
            for i in 0..k {
                let mut seen = FxHashMap::<_, usize>::default();
                for j in pos.indices() {
                    if !good[j] {
                        continue;
                    }
                    pos[j] = mv(pos[j].0, pos[j].1, s[i]);
                    if let Some(&o) = seen.get(&pos[j]) {
                        for x in 0..lens[o].min(lens[j]) {
                            joins.push(i + 1 + x * k);
                        }
                        let cand = lens[j];
                        lens[o].maxim(cand);
                        good.unset(j);
                    } else {
                        seen.insert(pos[j], j);
                    }
                }
            }
            lens.get(0).copied().unwrap_or(0)
        });
        rec.call(i);
        ans.push(None);
    }
    ans.pop();
    joins.sort_by_key(|&x| Reverse(x));
    ans.extend(joins.into_iter().map(Some));
    ans.resize(n * m, Some(0));
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
