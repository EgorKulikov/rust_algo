//{"name":"F. Mirror II","group":"Codeforces - TheForces Round #38 (Tree-Forces)","url":"https://codeforces.com/gym/105622/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n0 1 0 -1 0 0\n1 2 2 4 1\n","output":"9\n"},{"input":"10\n0 -7 0 5 7 0 -1 0 7 0\n9 2 1 8 2 5 1 8 3\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMirrorII"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
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
    let val = input.read_vec::<i128>(n);
    let p = input.read_size_vec(n - 1).dec();

    struct Distances {
        delta: i128,
        map: DefaultHashMap<i128, usize>,
    }

    impl Distances {
        fn add(&mut self, d: i128, by: usize) {
            self.map[d - self.delta] += by;
        }

        fn shift(&mut self, d: i128) {
            self.delta += d;
        }

        fn join(&mut self, mut other: Distances) {
            if self.map.len() < other.map.len() {
                std::mem::swap(self, &mut other);
            }
            for (k, v) in other.map {
                self.add(k + other.delta, v);
            }
        }

        fn get(&self, d: i128) -> usize {
            self.map[d - self.delta]
        }
    }

    let mut graph = Graph::new(n);
    for i in 1..n {
        graph.add_edge(Edge::new(p[i - 1], i));
    }
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|dfs, vert: usize, pr: i128| -> Distances {
        let mut distances = Vec::new();
        for e in &graph[vert] {
            let call = dfs.call(e.to(), pr + val[vert]);
            distances.push(call);
        }
        let l = val[vert];
        let b = 2 * (l + pr) - 1;
        let c = (l + pr) * (l + pr) + pr;

        let d = b * b - 4 * c;
        let roots = if d >= 0 {
            let mut dd = (d as f64).sqrt() as i128;
            while dd * dd < d {
                dd += 1;
            }
            while dd * dd > d {
                dd -= 1;
            }
            if dd * dd != d {
                vec![]
            } else {
                if dd == 0 {
                    vec![-b / 2]
                } else {
                    let x1 = (-b + dd) / 2;
                    let x2 = (-b - dd) / 2;
                    vec![x1, x2]
                }
            }
        } else {
            vec![]
        };
        let mut zeroes: usize = 0;
        let mut r: usize = 0;
        let is_zero_root = roots.contains(&(-(l + pr)));
        for dis in &distances {
            let cur_zeroes = dis.get(-(l + pr));
            let mut cur_r = 0;
            for x in roots.copy_iter() {
                if x != -(l + pr) {
                    cur_r += dis.get(x);
                }
            }
            ans += zeroes * cur_r;
            ans += cur_zeroes * r;
            if is_zero_root {
                ans += cur_zeroes * zeroes;
            }
            zeroes += cur_zeroes;
            r += cur_r;
        }
        let cur_zeroes = if l + pr == 0 { 1 } else { 0 };
        let mut cur_r = 0;
        for x in roots.copy_iter() {
            if x != -(l + pr) && x == 0 {
                cur_r += 1;
            }
        }
        ans += zeroes * cur_r;
        ans += cur_zeroes * r;
        if is_zero_root {
            ans += cur_zeroes * zeroes;
        }
        if distances.is_empty() {
            let mut dis = Distances {
                delta: 0,
                map: DefaultHashMap::new(),
            };
            dis.add(0, 1);
            dis.shift(l);
            return dis;
        }
        let mut dis = distances
            .iter_reduce(|mut a, b| {
                a.join(b);
                a
            })
            .unwrap();
        dis.add(0, 1);
        dis.shift(l);
        dis
    });
    dfs.call(0, 0);
    out.print_line(ans);
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

#[cfg(test)]
mod tester;
//END MAIN
