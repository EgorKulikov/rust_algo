//{"name":"task_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::time::Instant;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let start = Instant::now();
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut p = vec![0; n];
    let mut mat = Arr2d::new(n, n, 0);
    let mut rand = Random::new_with_seed(239);
    let mut val = vec![0; n];
    let mut delta = vec![0; n];
    loop {
        if start.elapsed().as_millis() >= 1900 {
            break;
        }
        mat.fill(0);
        for (u, v, w) in edges.copy_iter() {
            if w > p[v] - p[u] {
                mat[(u, v)] += 1;
            } else {
                mat[(u, v)] -= 1;
            }
            if w >= p[v] - p[u] {
                mat[(v, u)] -= 1;
            } else {
                mat[(v, u)] += 1;
            }
        }
        for i in 0..n {
            val[i] = rand.gen_range(0..2);
        }
        let mut value = 0;
        delta.fill(0);
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if val[i] == 0 && val[j] == 1 {
                    value += mat[(i, j)];
                    delta[i] -= mat[(i, j)];
                    delta[j] -= mat[(i, j)];
                }
                if val[i] == 1 && val[j] == 1 {
                    delta[i] += mat[(i, j)];
                }
                if val[i] == 0 && val[j] == 0 {
                    delta[j] += mat[(i, j)];
                }
            }
        }
        loop {
            let mut update = false;
            for i in 0..n {
                if delta[i] > 0 {
                    update = true;
                    value += delta[i];
                    for j in 0..n {
                        if i != j {
                            if val[i] == 0 && val[j] == 1 {
                                delta[i] += mat[(i, j)];
                                delta[j] += mat[(i, j)];
                            }
                            if val[i] == 1 && val[j] == 0 {
                                delta[i] += mat[(j, i)];
                                delta[j] += mat[(j, i)];
                            }
                            if val[i] == 1 && val[j] == 1 {
                                delta[i] -= mat[(i, j)];
                                delta[j] -= mat[(j, i)];
                            }
                            if val[i] == 0 && val[j] == 0 {
                                delta[j] -= mat[(i, j)];
                                delta[i] -= mat[(j, i)];
                            }
                        }
                    }
                    val[i] ^= 1;
                    for j in 0..n {
                        if i != j {
                            if val[i] == 0 && val[j] == 1 {
                                delta[i] -= mat[(i, j)];
                                delta[j] -= mat[(i, j)];
                            }
                            if val[i] == 1 && val[j] == 0 {
                                delta[i] -= mat[(j, i)];
                                delta[j] -= mat[(j, i)];
                            }
                            if val[i] == 1 && val[j] == 1 {
                                delta[i] += mat[(i, j)];
                                delta[j] += mat[(j, i)];
                            }
                            if val[i] == 0 && val[j] == 0 {
                                delta[j] += mat[(i, j)];
                                delta[i] += mat[(j, i)];
                            }
                        }
                    }
                }
            }
            if !update {
                break;
            }
        }
        if value > 0 {
            for i in 0..n {
                p[i] += val[i];
            }
        }
    }
    let mut ans = 0;
    for (u, v, w) in edges {
        ans += w.abs_diff(p[v] - p[u]);
    }
    out.print_line(ans);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
