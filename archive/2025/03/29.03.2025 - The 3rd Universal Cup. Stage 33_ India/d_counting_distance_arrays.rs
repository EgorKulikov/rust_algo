//{"name":"D. Counting Distance Arrays","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10268","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 2\n3\n1 2\n2 3\n7\n1 2\n2 3\n2 4\n3 5\n1 6\n5 7\n","output":"3\n6\n23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
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
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut d = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        d[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            d[vert].maxim(call + 1);
        }
        d[vert]
    });
    dfs.call(0, n);

    let mut ans = n;
    let mut dfs = RecursiveFunction3::new(|dfs, vert: usize, prev: usize, from_up: usize| {
        ans += from_up.min(d[vert]);
        let mut best = from_up;
        let mut second = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if best < d[e.to()] {
                second = best;
                best = d[e.to()];
            } else if second < d[e.to()] {
                second = d[e.to()];
            }
        }
        ans += second;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let new_from_up = if best == d[e.to()] {
                second + 1
            } else {
                best + 1
            };
            dfs.call(e.to(), vert, new_from_up);
        }
    });
    dfs.call(0, n, 0);
    type Mod = ModIntF;
    out.print_line(Mod::from_index(ans));
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
