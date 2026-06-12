use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let (l, r, p, q) = input.read_vec::<(i32, i32, i32, i32)>(n).detuple();
    let uvb = input.read_vec::<(usize, usize, i32)>(m).dec();

    if m + 1 != n {
        out.print_line(false);
        return;
    }
    let mut graph = Graph::new(n, n - 1);
    for (u, v, b) in uvb {
        graph.add_edge(Edge::with_payload(u, v, b));
    }
    let mut ok = true;
    let mut seen = 0;
    let mut rec = RecursiveFunction4::new(|rec, vert: usize, ep: i32, pl: i32, pr: i32| {
        if ep != p[vert] || pl >= l[vert] || pr <= r[vert] {
            ok = false;
            return;
        }
        seen += 1;
        let mut seen = Vec::new();
        for e in graph.adj(vert) {
            rec.call(e.to(), q[vert], l[vert], r[vert]);
            if !ok {
                return;
            }
            seen.push((*e.payload(), l[e.to()], r[e.to()]));
        }
        seen.sort();
        let mut last_b = -1;
        let mut last_l = l[vert];
        for (b, l, r) in seen {
            if b <= last_b || l <= last_l {
                ok = false;
                return;
            }
            last_b = b;
            last_l = r;
        }
    });
    rec.call(0, p[0], i32::MIN, i32::MAX);
    out.print_line(ok && seen == n);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
