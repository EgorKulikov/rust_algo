//{"name":"L. Label Matching","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14717","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6\n1 5 0 3 4 0\n0 3 4 5 2 0\n1 2\n2 3\n2 4\n1 5\n5 6\n5\n2 2 3 0 4\n4 1 4 2 0\n1 2\n2 3\n3 4\n4 5\n3\n1 2 3\n3 2 1\n1 2\n2 3\n","output":"111011\n01111\n100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::multi_set::MultiHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = Str::from(vec![b'0'; n]);
    struct State {
        set_a: MultiHashSet<usize>,
        set_b: MultiHashSet<usize>,
        wildcards_a: usize,
        wildcards_b: usize,
    }

    impl State {
        fn new(a: usize, b: usize) -> Self {
            let mut set_a = MultiHashSet::new();
            let mut set_b = MultiHashSet::new();
            if a != 0 && a != b {
                set_a.insert(a);
            }
            if b != 0 && a != b {
                set_b.insert(b);
            }
            Self {
                set_a,
                set_b,
                wildcards_a: if a == 0 { 1 } else { 0 },
                wildcards_b: if b == 0 { 1 } else { 0 },
            }
        }

        fn is_good(&self) -> bool {
            self.set_a.len() <= self.wildcards_b && self.set_b.len() <= self.wildcards_a
        }

        fn merge(&mut self, mut other: Self) {
            if self.set_a.len() + self.set_b.len() < other.set_a.len() + other.set_b.len() {
                std::mem::swap(self, &mut other);
            }
            for &key in other.set_a.iter() {
                if !self.set_b.remove(&key) {
                    self.set_a.insert(key);
                }
            }
            for &key in other.set_b.iter() {
                if !self.set_a.remove(&key) {
                    self.set_b.insert(key);
                }
            }
            self.wildcards_a += other.wildcards_a;
            self.wildcards_b += other.wildcards_b;
        }
    }

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> State {
        let mut cur = State::new(a[vert], b[vert]);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            cur.merge(call);
        }
        if cur.is_good() {
            ans[vert] = b'1';
        }
        cur
    });
    dfs.call(0, n);
    out.print_line(ans);
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
