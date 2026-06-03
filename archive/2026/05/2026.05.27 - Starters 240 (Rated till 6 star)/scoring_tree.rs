use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut leafs = MultiTreeSet::new();
    for i in 0..n {
        if graph.degree(i) == 1 {
            let mut len = 1;
            let mut cur = graph.adj(i).iter().next().unwrap().to();
            let mut prev = i;
            while graph.degree(cur) == 2 {
                let sum = graph.adj(cur).iter().map(|e| e.to()).sum::<usize>();
                let next = sum - prev;
                prev = cur;
                cur = next;
                len += 1;
            }
            leafs.insert(len);
        }
    }
    let mut safe = 0;
    while leafs.len() > 1 {
        let max = *leafs.last().unwrap();
        leafs.remove(&max);
        let max = *leafs.last().unwrap();
        leafs.remove(&max);
        if max > 1 {
            leafs.insert(max - 1);
        }
        safe += 1;
    }
    out.print_line((n / 2).saturating_sub(safe));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
