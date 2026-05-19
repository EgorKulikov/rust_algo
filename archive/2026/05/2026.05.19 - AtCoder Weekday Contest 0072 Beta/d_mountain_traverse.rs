//{"name":"D - Mountain Traverse","group":"AtCoder - AtCoder Weekday Contest 0072 Beta","url":"https://atcoder.jp/contests/awc0072/tasks/awc0072_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n10 20 30 40 5\n1 2\n2 3\n3 4\n1 3\n","output":"4\n"},{"input":"4 4\n100 50 30 200\n1 2\n1 3\n2 4\n3 4\n","output":"1\n"},{"input":"8 9\n15 30 10 45 25 50 35 60\n1 2\n1 5\n2 4\n5 7\n7 4\n4 6\n6 8\n1 3\n3 5\n","output":"6\n"},{"input":"15 20\n5 50 10 45 15 40 20 35 25 30 55 60 65 70 75\n1 3\n3 5\n5 7\n7 9\n9 10\n10 11\n11 12\n12 13\n13 14\n14 15\n1 2\n2 4\n4 6\n6 8\n1 5\n3 7\n5 9\n9 11\n7 10\n2 6\n","output":"11\n"},{"input":"1 0\n1000000000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let mut mem = Memoization1d::new(n, |mem, cur| -> usize {
        let mut res = 1;
        for e in graph.adj(cur) {
            if p[e.to()] > p[cur] {
                res.maxim(1 + mem.call(e.to()));
            }
        }
        res
    });
    out.print_line(mem.call(0));
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
