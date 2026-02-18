//{"name":"task_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    type Mod = ModIntF;
    let graph = Graph::with_biedges(n, &edges);
    for i in 0..n {
        if graph[i].len() >= 3 {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Mod, Mod) {
                let mut all = Mod::from(1);
                let mut but_one = Mod::from(0);
                let mut down = 0;
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    let (call_has, call_has_not) = f.call(e.to(), vert);
                    let next_but_one = but_one * call_has + all * call_has_not;
                    all *= call_has;
                    but_one = next_but_one;
                    down += 1;
                }
                if down == 0 {
                    (Mod::from(1), Mod::from(1))
                } else if down == 1 {
                    (all * 2 + but_one, but_one)
                } else {
                    ((all + but_one) * 2, Mod::from(0))
                }
            });
            out.print_line(dfs.call(i, n).0);
            return;
        }
    }
    out.print_line(Mod::new(2).power(n) - (n - 2) - 1);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
