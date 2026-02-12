//{"name":"K. The Alarm","group":"Codeforces - Practice Contest","url":"https://constructor2026.contest.codeforces.com/group/XdjJUfzFUt/contest/668785/problem/K","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 4 5\n5 5 0\n3 3 2\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::geometry::circle::Circle;
use algo_lib::geometry::point::Point;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let xyp = input.read_vec::<(i64, i64, i64)>(n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        let (x, y, p) = xyp[i];
        let c = Circle::new(Point::new(x, y), p);
        for j in 0..n {
            let (x, y, _) = xyp[j];
            if i != j && c.contains(Point::new(x, y)) {
                graph.add_edge(Edge::new(i, j));
            }
        }
    }
    let scc = graph.strongly_connected_components();
    type Mod = ModIntF;
    let mut has_incoming = BitSet::new(scc.condensed.vertex_count());
    for (_, e) in scc.condensed.edges() {
        has_incoming.set(e.to());
    }
    let qty = scc.color.qty_bound(scc.condensed.vertex_count());
    let mut ans = Mod::new(1);
    for i in qty.indices() {
        if has_incoming[i] {
            ans *= Mod::new(2).power(qty[i]);
        } else {
            ans *= Mod::new(2).power(qty[i]) - 1;
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
