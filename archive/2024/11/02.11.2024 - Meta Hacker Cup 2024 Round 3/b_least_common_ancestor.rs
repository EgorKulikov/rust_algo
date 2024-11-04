//{"name":"B: Least Common Ancestor","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-3/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2\n-1 alice\n1 bob\n8\n-1 alice\n1 alice\n2 bob\n3 carl\n3 anna\n1 bob\n6 alice\n7 anna\n10\n-1 alice\n1 bob\n1 emily\n2 alice\n3 alice\n3 abby\n4 carl\n5 abby\n7 dan\n7 emily\n","output":"Case #1: 21\nCase #2: 243150762\nCase #3: 255661373\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"least_common_ancestor_.*input[.]txt"},"output":{"type":"file","fileName":"least_common_ancestor_output.txt","pattern":null},"languages":{"java":{"taskClass":"BLeastCommonAncestor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::string::str::Str;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let people = input.read_vec::<(isize, Str)>(n);
    drop(input);

    let graph = Graph::new(n).do_with(|graph| {
        for (i, &(p, _)) in people.iter().enumerate().skip(1) {
            graph.add_edge(Edge::new(p as usize - 1, i));
        }
    });
    let mut u = Vec::with_capacity(n);
    for (_, name) in &people {
        u.push(name);
    }
    u.sort_unstable();
    u.dedup();
    let mut id = Vec::with_capacity(n);
    for (_, name) in &people {
        id.push(u.lower_bound(&name) + 1);
    }
    #[derive(Default, Debug)]
    struct MostFrequent {
        qty: DefaultHashMap<usize, usize>,
        by_qty: BTreeSet<(usize, usize)>,
    }
    impl MostFrequent {
        fn add(&mut self, x: usize) {
            let qty = self.qty[x];
            self.by_qty.remove(&(qty, x));
            self.qty[x] += 1;
            self.by_qty.insert((self.qty[x], x));
        }
        fn remove(&mut self, x: usize) {
            let qty = self.qty[x];
            self.by_qty.remove(&(qty, x));
            self.qty[x] -= 1;
            if self.qty[x] > 0 {
                self.by_qty.insert((self.qty[x], x));
            }
        }
        fn most_frequent(&self) -> usize {
            self.by_qty.iter().next().map(|&(_, x)| x).unwrap_or(0)
        }
        fn join(mut left: Self, mut right: Self) -> Self {
            if left.qty.len() < right.qty.len() {
                std::mem::swap(&mut left, &mut right);
            }
            for (x, qty) in right.qty {
                for _ in 0..qty {
                    left.add(x);
                }
            }
            left
        }
    }
    let mut parents = MostFrequent::default();
    let mut a = vec![0; n];
    let mut d = vec![0; n];
    let mut dfs = RecursiveFunction::new(|rec, vert: usize| {
        a[vert] = parents.most_frequent();
        parents.add(id[vert]);
        let mut children = MostFrequent::default();
        for e in &graph[vert] {
            let call = rec.call(e.to());
            children = MostFrequent::join(children, call);
        }
        d[vert] = children.most_frequent();
        children.add(id[vert]);
        parents.remove(id[vert]);
        children
    });
    dfs.call(0);
    type Mod = ModIntF;
    let mut ans = Mod::zero();
    for i in 0..n {
        ans = ans * Mod::from_index(u.len() + 1) + Mod::from_index(a[i]);
        ans = ans * Mod::from_index(u.len() + 1) + Mod::from_index(d[i]);
    }
    // eprintln!("a = {:?}", a);
    // eprintln!("d = {:?}", d);
    // eprintln!("all = {:?}", all);
    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
