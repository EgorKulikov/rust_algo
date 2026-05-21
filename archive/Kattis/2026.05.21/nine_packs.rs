use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let hq = input.read_size();
    let h = input.read_size_vec(hq);
    let bq = input.read_size();
    let b = input.read_size_vec(bq);

    fn build(h: &[usize]) -> Vec<Option<usize>> {
        let sum = h.iter().sum::<usize>();
        let mut ans = vec![None; sum + 1];
        ans[0] = Some(0);
        for &x in h {
            for i in (0..=sum - x).rev() {
                if let Some(v) = ans[i] {
                    ans[i + x].minim(v + 1);
                }
            }
        }
        ans
    }
    let h = build(&h);
    let b = build(&b);
    let mut ans = None;
    for i in 1..h.len().min(b.len()) {
        if let (Some(h), Some(b)) = (h[i], b[i]) {
            ans.minim(h + b);
        }
    }
    match ans {
        Some(ans) => out.print_line(ans),
        None => out.print_line("impossible"),
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
