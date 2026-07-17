use algo_lib::collections::bit_set::BitSet;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut g_pos = Graph::new_2d(n);
    let mut g_neg = Graph::new_2d(n);

    for _ in 0..m {
        let o = input.read_int();
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        if o == 1 {
            g_pos.add_edge(BiEdge::new(u, v));
        } else {
            g_neg.add_edge(BiEdge::new(u, v));
        }
    }
    let mut d_pos = g_pos.degrees();
    let mut d_neg = g_neg.degrees();
    let mut cur = n as i32;
    let mut ans = vec![0; n];
    let mut queue = VecDeque::new();
    let mut added = BitSet::new(n);
    for i in 0..n {
        if d_pos[i] == 0 || d_neg[i] == 0 {
            queue.push_back(i);
            added.set(i);
        }
    }
    while let Some(v) = queue.pop_front() {
        if d_pos[v] == 0 {
            ans[v] = -cur;
        } else {
            ans[v] = cur;
        }
        cur -= 1;
        for e in g_pos.adj(v) {
            if !added[e.to()] {
                d_pos[e.to()] -= 1;
                if d_pos[e.to()] == 0 {
                    added.set(e.to());
                    queue.push_back(e.to());
                }
            }
        }
        for e in g_neg.adj(v) {
            if !added[e.to()] {
                d_neg[e.to()] -= 1;
                if d_neg[e.to()] == 0 {
                    added.set(e.to());
                    queue.push_back(e.to());
                }
            }
        }
    }
    if cur != 0 {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    out.print_line(ans);
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
