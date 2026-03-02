//{"name":"E - Optimal Route for a Sightseeing Tour","group":"AtCoder - AtCoder Weekday Contest 0016 Beta","url":"https://atcoder.jp/contests/awc0016/tasks/awc0016_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n10 20 30\n1 3\n1 2 5\n2 3 5\n1 3 25\n","output":"50\n"},{"input":"5 7\n10 20 30 40 50\n1 5\n1 2 3\n1 4 10\n2 3 4\n2 4 7\n2 5 3\n3 5 8\n4 5 6\n","output":"126\n"},{"input":"8 10\n5 50 50 50 50 50 50 5\n1 8\n1 2 2\n1 8 100\n2 3 3\n3 4 3\n4 5 1\n4 8 4\n5 6 1\n6 7 1\n7 8 5\n2 8 50\n","output":"294\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_int_vec(n);
    let s = input.read_size() - 1;
    let t = input.read_size() - 1;
    let edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut ans = Arr2d::new(1 << n, n, None);
    ans[(1 << s, s)] = Some(p[s]);
    let mut res = None;
    for i in usize::iter_all(n) {
        loop {
            let mut updated = false;
            for j in 0..n {
                if let Some(val) = ans[(i, j)] {
                    for e in &graph[j] {
                        let to = e.to();
                        if i.is_set(to) {
                            updated |= ans[(i, to)].maxim(val - e.weight());
                        } else {
                            ans[(i.with_bit(to), to)].maxim(val + p[to] - e.weight());
                        }
                    }
                }
            }
            if !updated {
                break;
            }
        }
        if let Some(cand) = ans[(i, t)] {
            res.maxim(cand);
        }
    }
    out.print_line(res);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
