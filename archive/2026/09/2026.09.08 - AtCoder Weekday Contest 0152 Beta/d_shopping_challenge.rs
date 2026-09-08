use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long();
    let p = input.read_long_vec(n);

    fn build(p: &[i64], s: i64) -> Vec<(i64, bool)> {
        let mut res = Vec::new();
        for i in usize::iter_all(p.len()) {
            let mut cur = 0i64;
            for j in p.indices() {
                if i.is_set(j) {
                    cur += p[j];
                    if cur > s {
                        break;
                    }
                }
            }
            if cur <= s {
                res.push(cur);
            }
        }
        res.sort();
        let mut ans = vec![(res[0], false)];
        for v in res.copy_skip(1) {
            if ans[Back(0)].0 == v {
                ans[Back(0)].1 = true;
            } else {
                ans.push((v, false));
            }
        }
        ans
    }
    let left = build(&p[..n / 2], s);
    let right = build(&p[n / 2..], s);
    let mut ans = 0;
    let mut at = right.len();
    for (l, multi) in left {
        while at > 0 && l + right[at - 1].0 > s {
            at -= 1;
        }
        if at > 0 && l + right[at - 1].0 == s {
            if multi || right[at - 1].1 {
                ans += 2;
            } else {
                ans += 1;
            }
        }
    }
    out.print_line(match ans {
        0 => "NO",
        1 => "ALMOST",
        _ => "YES",
    });
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
