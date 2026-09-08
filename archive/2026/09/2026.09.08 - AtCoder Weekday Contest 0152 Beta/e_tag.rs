use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let direct = Graph::with_edges(n, &edges);
    let mut graph = Graph::new(n, m);
    for (u, v) in edges {
        graph.add_edge(Edge::new(v, u));
    }
    let mut rem = Arr2d::with_gen(n, n, |i, j| {
        if i == j {
            0
        } else {
            direct.degree(i) + 1
        }
    });
    let mut aoki = Arr2d::new(n, n, false);
    let mut takahashi = Arr2d::new(n, n, false);
    let mut queue = Vec::new();
    for i in 0..n {
        aoki[(i, i)] = true;
        queue.push((i, i));
    }
    while let Some((i, j)) = queue.pop() {
        let mut to_process = Vec::new();
        if !takahashi[(j, i)] {
            to_process.push((j, i));
            takahashi[(j, i)] = true;
        }
        for e in graph.adj(j) {
            if !takahashi[(e.to(), i)] {
                to_process.push((e.to(), i));
                takahashi[(e.to(), i)] = true;
            }
        }
        for (i, j) in to_process {
            if !aoki[(j, i)] {
                rem[(j, i)] -= 1;
                if rem[(j, i)] == 0 {
                    aoki[(j, i)] = true;
                    queue.push((j, i));
                }
            }
            for e in graph.adj(j) {
                if !aoki[(e.to(), i)] {
                    rem[(e.to(), i)] -= 1;
                    if rem[(e.to(), i)] == 0 {
                        aoki[(e.to(), i)] = true;
                        queue.push((e.to(), i));
                    }
                }
            }
        }
    }
    /*loop {
        let mut updated = false;
        for i in 0..n {
            for j in 0..n {
                if aoki[(i, j)] {
                    continue;
                }
                if takahashi[(j, i)] {
                    aoki[(i, j)] = true;
                    continue;
                }
                for e in graph.adj(i) {
                    if takahashi[(j, e.to())] {
                        aoki[(i, j)] = true;
                        updated = true;
                        break;
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if takahashi[(i, j)] || !aoki[(j, i)] {
                    continue;
                }
                let mut good = true;
                for e in graph.adj(i) {
                    if !aoki[(j, e.to())] {
                        good = false;
                        break;
                    }
                }
                if good {
                    takahashi[(i, j)] = true;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }*/

    let q = input.read_size();
    for _ in 0..q {
        let s = input.read_size() - 1;
        let r = input.read_size() - 1;
        out.print_line(!aoki[(s, r)]);
    }
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
