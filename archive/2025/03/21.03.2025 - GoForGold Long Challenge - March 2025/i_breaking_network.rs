//{"name":"I. Breaking Network","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/I","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n1 2\n1 3\n1 1 1 0\n0 0 1 1\n1 1 0 0\n","output":"2\n"},{"input":"2 5\n1 2\n1 0 0 1 0\n0 0 1 0 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let keys = input.read_int_table(n, k);

    let kt = Vec::with_gen(n, |i| {
        let mut res = 0;
        for j in 0..k {
            if keys[(i, j)] == 1 {
                res.set_bit(j);
            }
        }
        res
    });
    let mut ans = 0;
    let graph = Graph::with_biedges(n, &edges);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Vec<i64> {
        let mut res = vec![0; 1 << k];
        res[kt[vert]] = 1;
        if kt[vert] == usize::all_bits(k) {
            ans += 1;
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            for i in res.indices() {
                let other = usize::all_bits(k) - i;
                for j in BitIter::new(i) {
                    ans += res[i] * call[other + j];
                }
            }
            for i in res.indices() {
                res[i | kt[vert]] += call[i];
            }
        }
        res
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
