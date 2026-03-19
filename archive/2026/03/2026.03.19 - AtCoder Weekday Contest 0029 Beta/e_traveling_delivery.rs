//{"name":"E - Traveling Delivery","group":"AtCoder - AtCoder Weekday Contest 0029 Beta","url":"https://atcoder.jp/contests/awc0029/tasks/awc0029_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 3\n2 3 2\n3 1 4\n1 3 10\n2 1 5\n1 2\n2 3\n","output":"9\n"},{"input":"3 2\n1 2 1\n2 3 1\n1 1\n2\n","output":"-1\n"},{"input":"6 10\n1 2 5\n1 3 8\n2 4 3\n3 4 2\n4 5 4\n5 6 1\n6 1 7\n4 1 10\n5 3 6\n2 6 9\n1 3\n2 5 6\n","output":"20\n"},{"input":"10 20\n1 2 4\n2 3 3\n3 4 5\n4 5 2\n5 6 7\n6 7 1\n7 8 6\n8 9 3\n9 10 4\n10 1 8\n1 5 20\n5 1 15\n2 6 10\n6 2 12\n3 7 8\n7 3 9\n4 8 6\n8 4 7\n1 3 11\n9 1 5\n1 5\n2 4 6 8 10\n","output":"43\n"},{"input":"2 2\n1 2 1000000\n2 1 1000000\n1 1\n2\n","output":"2000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::all_distances::{AllDistances, Distance};
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
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
    let uvw = input.read_vec::<(usize, usize, i64)>(m).dec();
    let s = input.read_size() - 1;
    let k = input.read_size();
    let t = input.read_size_vec(k).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in uvw {
        graph.add_edge(WeightedEdge::new(u, v, w));
    }
    let d = graph.all_distances();

    for i in t.copy_iter() {
        if matches!(d[(s, i)], Distance::None) || matches!(d[(i, s)], Distance::None) {
            out.print_line(-1);
            return;
        }
    }
    let mut ans = Arr2d::new(k, 1 << k, None);
    for i in 0..k {
        ans[(i, 1 << i)] = Some(d[(s, t[i])].to_finite());
    }
    for mask in usize::iter_all(k) {
        for i in 0..k {
            if let Some(val) = ans[(i, mask)] {
                for j in 0..k {
                    if !mask.is_set(j) {
                        ans[(j, mask.with_bit(j))].minim(val + d[(t[i], t[j])].to_finite());
                    }
                }
            }
        }
    }
    let mut res = None;
    for i in 0..k {
        res.minim(ans[(i, usize::all_bits(k))].unwrap() + d[(t[i], s)].to_finite());
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
