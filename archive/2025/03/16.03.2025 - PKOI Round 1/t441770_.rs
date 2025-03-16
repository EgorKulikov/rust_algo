//{"name":"T441770 移动迷宫","group":"Luogu","url":"https://www.luogu.com.cn/problem/T441770?contestId=183510","interactive":false,"timeLimit":1000,"tests":[{"input":"5 7\n1 2 3\n2 3 8\n3 5 1000\n2 4 100\n4 5 6\n1 4 78888\n1 3 114514\n","output":"151073832\n"},{"input":"6 8\n1 3 100000000\n1 5 200000000\n2 5 300000000\n2 6 400000000\n3 4 500000000\n5 6 600000000\n4 5 700000000\n3 6 303063652\n","output":"403063652\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, u32)>(m).dec();

    value!(Modulo: u32 = 754974721);
    type Mod = ModInt<Modulo>;
    let graph = Graph::new(3 * n).do_with(|graph| {
        for (u, v, w) in edges {
            let w = Mod::new(w);
            graph.add_edge(WeightedEdge::new(u, v + n, w.val() as i64));
            graph.add_edge(WeightedEdge::new(v, u + n, w.val() as i64));
            graph.add_edge(WeightedEdge::new(
                u + n,
                v + 2 * n,
                (Mod::one() / (Mod::one() - w)).val() as i64,
            ));
            graph.add_edge(WeightedEdge::new(
                v + n,
                u + 2 * n,
                (Mod::one() / (Mod::one() - w)).val() as i64,
            ));
            graph.add_edge(WeightedEdge::new(
                u + 2 * n,
                v,
                ((w - Mod::one()) / w).val() as i64,
            ));
            graph.add_edge(WeightedEdge::new(
                v + 2 * n,
                u,
                ((w - Mod::one()) / w).val() as i64,
            ));
        }
    });
    let mut ans = None;
    for i in (n - 1..).step_by(n).take(3) {
        if let Some((d, _)) = graph.distance(0, i) {
            ans.minim(d);
        }
    }
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
