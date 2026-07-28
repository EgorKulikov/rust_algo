use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let uv = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &uv);
    let mut step = Vec::new();
    
    'outer:
    for i in 0..n {
        let mut res = vec![None; n];
        let mut queue = VecDeque::new();
        queue.push_back(i);
        res[i] = Some(0);
        while let Some(vert) = queue.pop_front() {
            let cur = res[vert].unwrap();
            for e in graph.adj(vert) {
                if res[e.to()].minim(cur + 1) {
                    queue.push_back(e.to());
                }
            }
        }
        let mut r = 0;
        for j in 0..n {
            match res[j] {
                Some(x) => {
                    r.maxim(x);
                }
                None => {
                    continue 'outer;
                }
            }
        }
        step.push(r);
    }
    step.sort();

    for _ in 0..q {
        let k = input.read_size();
        out.print_line(step.less_or_eq(&k));
    }
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
