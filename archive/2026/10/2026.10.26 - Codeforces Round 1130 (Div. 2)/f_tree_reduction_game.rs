use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    #[derive(Clone)]
    enum State {
        None,
        ButOne(usize),
        All,
    }
    let mut state = vec![State::None; n];
    let mut q_lose = vec![0; n];
    let mut mem = Memoization2::new(|mem, vert: usize, prev: usize| -> bool {
        match state[vert] {
            State::None => {
                for e in graph.adj(vert) {
                    if e.to() != prev {
                        if !mem.call(e.to(), vert) {
                            q_lose[vert] += 1;
                        }
                    }
                }
                if prev == n {
                    state[vert] = State::All;
                } else {
                    state[vert] = State::ButOne(prev);
                }
            }
            State::ButOne(was_prev) => {
                if prev != was_prev {
                    if !mem.call(was_prev, vert) {
                        q_lose[vert] += 1;
                    }
                    state[vert] = State::All;
                }
            }
            State::All => {}
        }
        match state[vert] {
            State::All => {
                if prev == n {
                    q_lose[vert] > 0 || graph.degree(vert) % 2 == 1
                } else {
                    let mut ql = q_lose[vert];
                    if !mem.call(prev, vert) {
                        ql -= 1;
                    }
                    ql > 0 || graph.degree(vert) % 2 == 1
                }
            }
            State::ButOne(was_prev) => {
                assert_eq!(prev, was_prev);
                q_lose[vert] > 0 || graph.degree(vert) % 2 == 1
            }
            State::None => unreachable!(),
        }
    });
    let mut ans = Vec::new();
    for i in 0..n {
        if !mem.call(i, n) {
            ans.push(i + 1);
        }
    }
    if ans.is_empty() {
        out.print_line("Alice");
    } else {
        out.print_line("Bob");
        out.print_line(ans.len());
        out.print_line(ans);
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
