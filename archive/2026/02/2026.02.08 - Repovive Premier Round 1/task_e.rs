//{"name":"task_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Arr3d<Option<i64>> {
        let mut ans = Arr3d::new(2, 3, 2, None);
        ans[(0, 0, 0)] = Some(a[vert]);
        ans[(1, 1, 1)] = Some(0);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            let mut next = Arr3d::new(2, 3, 2, None);
            for a_rem in 0..2 {
                for a_q in 0..3 {
                    for a_o in 0..2 {
                        if ans[(a_rem, a_q, a_o)].is_none() {
                            continue;
                        }
                        let a_val = ans[(a_rem, a_q, a_o)].unwrap();
                        for c_rem in 0..2 {
                            for c_q in 0..3 {
                                for c_o in 0..2 {
                                    if call[(c_rem, c_q, c_o)].is_none() {
                                        continue;
                                    }
                                    let c_val = call[(c_rem, c_q, c_o)].unwrap();
                                    if a_rem == 0 && c_rem == 0 {
                                        if c_o == 0 && (c_q == 0 || c_q == 2) {
                                            next[(0, a_q, a_o)].maxim(a_val + c_val);
                                        }
                                    } else if a_rem == 1 && c_rem == 1 {
                                        next[(1, (a_q + c_q - 1).min(2), (a_o + c_o) % 2)]
                                            .maxim(a_val + c_val);
                                    } else if a_rem == 0 && c_rem == 1 {
                                        if c_o == 0 && c_q == 2 {
                                            next[(0, a_q, a_o)].maxim(a_val + c_val);
                                        }
                                        next[(0, (a_q + c_q).min(2), (a_o + c_o) % 2)]
                                            .maxim(a_val + c_val);
                                    } else {
                                        next[(1, (a_q + c_q).min(2), (a_o + c_o) % 2)]
                                            .maxim(a_val + c_val);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ans = next;
        }
        ans
    });
    let call = dfs.call(0, 0);
    let mut ans = None;
    if let Some(val) = call[(0, 0, 0)] {
        ans.maxim(val);
    }
    if let Some(val) = call[(0, 2, 0)] {
        ans.maxim(val);
    }
    if let Some(val) = call[(1, 2, 0)] {
        ans.maxim(val);
    }
    out.print_line(ans);
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
