//{"name":"E. Ancient Tree","group":"Codeforces - Atto Round 1 (Codeforces Round 1041, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2127/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4 4\n5 5 5 5\n1 0 2 3\n1 2\n1 3\n1 4\n5 2\n3 1 4 1 5\n1 2 1 2 2\n1 4\n2 1\n3 4\n4 5\n11 3\n3 1 4 3 1 4 3 1 4 5 6\n0 0 0 2 1 2 1 2 2 1 1\n1 2\n2 3\n2 4\n2 5\n2 6\n1 7\n7 8\n7 9\n10 3\n3 11\n4 3\n2 3 2 3\n2 1 0 0\n3 1\n1 2\n2 4\n","output":"0\n1 4 2 3\n3\n1 2 1 2 2\n7\n2 3 1 2 1 2 1 2 2 1 1\n0\n2 1 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _k = input.read_size();
    let w = input.read_long_vec(n);
    let mut c = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    #[derive(Default)]
    struct Set {
        set: FxHashSet<usize>,
        vec: Vec<usize>,
    }

    impl Set {
        fn join(mut self, other: Self) -> (Self, Vec<usize>) {
            if self.vec.len() < other.vec.len() {
                other.join(self)
            } else {
                let mut same = Vec::new();
                for v in other.vec {
                    if self.set.contains(&v) {
                        same.push(v);
                    } else {
                        self.set.insert(v);
                        self.vec.push(v);
                    }
                }
                (self, same)
            }
        }

        fn new(x: usize) -> Self {
            let mut set = FxHashSet::default();
            set.insert(x);
            Set { set, vec: vec![x] }
        }
    }

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Set {
        let mut res = Set::default();
        let mut same = Vec::new();
        let mut num_children = 0;
        let mut empty_children = Vec::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            num_children += 1;
            let call = f.call(e.to(), vert);
            if call.vec.is_empty() {
                empty_children.push(e.to());
            }
            let (n_res, n_same) = res.join(call);
            res = n_res;
            same.extend(n_same);
        }
        let mut bad = false;
        for i in 1..same.len() {
            if same[i] != same[i - 1] {
                bad = true;
                break;
            }
        }
        if c[vert] != 0 && !same.is_empty() && c[vert] != same[0] {
            bad = true;
        }
        if bad && num_children > 1 {
            ans += w[vert];
        }
        if c[vert] != 0 {
            res = res.join(Set::new(c[vert])).0;
        }
        if !res.vec.is_empty() {
            if c[vert] == 0 {
                c[vert] = if !same.is_empty() {
                    same[0]
                } else {
                    res.vec[0]
                };
            }
            let root = vert;
            for v in empty_children {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    assert_eq!(c[vert], 0);
                    c[vert] = c[root];
                    for e in &graph[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        f.call(e.to(), vert);
                    }
                });
                dfs.call(v, vert);
            }
        }
        res
    });
    dfs.call(0, n);
    if c[0] == 0 {
        c.fill(1);
    }
    out.print_line(ans);
    out.print_line(c);
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
