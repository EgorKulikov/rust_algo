use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut ans = 0;
    for _ in 0..m {
        let k = input.read_size();
        let a = input.read_size_vec(k).dec();
        if 2 * k > n {
            let mut pos = vec![Vec::new(); n - k + 1];
            for i in 0..k {
                if a[i] >= i && a[i] <= n - k + i {
                    let delta = a[i] - i;
                    pos[delta].push(i);
                }
            }
            let mut res = pos[n - k].clone().reversed();
            for i in (0..n - k).rev() {
                let cur = pos[i].clone().reversed();
                let mut at = 0;
                for i in res.indices() {
                    while at < cur.len() && cur[at] > res[i] {
                        at += 1;
                    }
                    if at == cur.len() {
                        res.truncate(i);
                        break;
                    }
                    res[i] = cur[at];
                    at += 1;
                }
            }
            ans += res.len();
            /*while left < right {
                let mid = (left + right) / 2;
                let mut at = 0;
                let mut qty = 0;
                for i in 0..k {
                    if a[i] >= i && a[i] <= n - k + i {
                        let delta = a[i] - i;
                        if delta == at {
                            qty += 1;
                            if qty == mid + 1 {
                                at += 1;
                                qty = 0;
                            }
                        }
                    }
                }
                if at == n - k + 1 {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            ans += left;*/
        }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
