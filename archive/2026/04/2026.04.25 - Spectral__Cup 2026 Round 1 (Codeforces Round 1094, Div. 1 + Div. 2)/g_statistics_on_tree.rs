//{"name":"G. Statistics on Tree","group":"Codeforces - Spectral::Cup 2026 Round 1 (Codeforces Round 1094, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2222/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n2\n1 2\n3\n1 2\n1 3\n4\n1 2\n2 3\n3 4\n5\n1 2\n2 3\n2 4\n2 5\n7\n3 4\n1 5\n2 3\n2 6\n5 2\n7 3\n8\n2 1\n3 2\n4 1\n5 3\n6 2\n7 6\n8 1\n","output":"1\n1 2\n1 2 3\n1 3 2 4\n0 0 6 4 5\n0 4 5 5 3 4 7\n0 0 12 4 3 5 4 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut sz = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        sz[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            sz[vert] += sz[e.to()];
        }
    });
    dfs.call(0, 0);

    struct Node {
        val: usize,
        self_qty: usize,
        qty: usize,
        to_add: usize,
        to_add_delta: usize,
        to_add_bonus: usize,
    }
    impl Node {
        fn new(val: usize, qty: usize) -> Self {
            Self {
                val,
                self_qty: qty,
                qty,
                to_add: 0,
                to_add_delta: 0,
                to_add_bonus: 0,
            }
        }
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.qty = self.self_qty + left.map_or(0, |x| x.qty) + right.map_or(0, |x| x.qty);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.to_add += delta.to_add_delta;
            self.to_add_delta += delta.to_add_delta;
        }

        fn reset_delta(&mut self) {
            self.to_add_delta = 0;
        }
    }
    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.val
        }

        fn union(a: Self, b: Self) -> Self {
            Self {
                val: a.val,
                qty: a.self_qty + b.self_qty,
                self_qty: a.self_qty + b.self_qty,
                to_add: 0,
                to_add_delta: 0,
                to_add_bonus: a.to_add_bonus
                    + b.to_add_bonus
                    + a.self_qty * a.to_add
                    + b.self_qty * b.to_add,
            }
        }
    }

    let mut ans = vec![0; n];
    ans[n - 1] = n;
    let mut add_impl = |a: &mut Tree<Node>, b: &mut Tree<Node>, min: usize| {
        let small_qty_a = a.range(..=&min).payload().map_or(0, |x| x.qty);
        let small_qty_b = b.range(..=&min).payload().map_or(0, |x| x.qty);
        ans[min] += small_qty_a * small_qty_b;
        b.range(&(min + 1)..).with_payload_mut(|p| {
            p.to_add_delta += small_qty_a;
            p.to_add += small_qty_a;
        });
        for node in a.range(&(min + 1)..).iter() {
            let cur_val = node.val.max(min);
            let cur_qty = node.self_qty;
            let less_qty = b.range(..=&cur_val).payload().map_or(0, |x| x.qty);
            ans[cur_val] += cur_qty * less_qty;
            b.range(&(cur_val + 1)..).with_payload_mut(|p| {
                p.to_add_delta += cur_qty;
                p.to_add += cur_qty;
            });
        }
    };
    let mut add = |a: &mut Tree<Node>, b: &mut Tree<Node>, min: usize| {
        if a.size() <= b.size() {
            add_impl(a, b, min);
        } else {
            add_impl(b, a, min);
        }
    };
    let mut ans2 = vec![0; n];

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Tree<Node> {
        let mut down = FxHashMap::default();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            let x = sz[e.to()];
            for (&k, v) in down.iter_mut() {
                add(v, &mut call, n - k - x - 1);
            }
            if let Some(v) = down.remove(&x) {
                call = Tree::union(call, v);
            }
            down.insert(x, call);
        }
        let mut res = Tree::single(Node::new(sz[vert] - 1, 1));
        for (k, mut v) in down {
            let min = n - k - 1;
            let qty = v.range(..=&min).payload().map_or(0, |x| x.qty);
            ans2[min] += qty;
            v.range(&(min + 1)..).with_payload_mut(|p| {
                p.to_add_delta += 1;
                p.to_add += 1;
            });
            let min = sz[vert] - k - 1;
            let (mut front, back) = v.split_inclusive(&min);
            res = Tree::union(res, back);
            let mut total = 0;
            for node in front.iter() {
                ans2[node.val] += node.to_add_bonus + node.self_qty * node.to_add;
                total += node.self_qty;
            }
            res = Tree::union(res, Tree::single(Node::new(min, total)));
        }
        res
    });
    let mut res = dfs.call(0, 0);
    for node in res.iter() {
        ans2[node.val] += node.to_add_bonus + node.self_qty * node.to_add;
    }
    for i in 0..n {
        ans[i] += ans2[i];
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
