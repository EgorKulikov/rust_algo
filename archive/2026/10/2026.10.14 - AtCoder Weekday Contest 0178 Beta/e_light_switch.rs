use algo_lib::collections::bit_set::BitSet;
use algo_lib::graph::Graph;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
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
    let k = input.read_size();
    let mut b = Vec::with_capacity(m);
    let mut c = Vec::with_capacity(m);
    for _ in 0..m {
        let mut cur = 0;
        for i in 0..n {
            if input.read_int() == 1 {
                cur.set_bit(i);
            }
        }
        b.push(cur);
        c.push(input.read_long());
    }
    let mut is_f = BitSet::new(1 << n);
    for _ in 0..k {
        let mut cur = 0;
        for i in 0..n {
            if input.read_int() == 1 {
                cur.set_bit(i);
            }
        }
        is_f.set(cur);
    }

    let mut graph = Graph::new_2d(1 << n);
    for i in usize::iter_all(n) {
        if is_f[i] {
            continue;
        }
        for j in 0..m {
            graph.add_edge(WeightedEdge::new(i, i ^ b[j], c[j]));
        }
    }
    out.print_line(graph.distance(0, usize::all_bits(n)).map(|x| x.0));
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
