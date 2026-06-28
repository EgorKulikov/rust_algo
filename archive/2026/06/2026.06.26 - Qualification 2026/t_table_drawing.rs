use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let t = input.read_char_table(2 * r + 1, 2 * c + 1);

    fn calc(t: &Arr2d<u8>) -> Vec<(usize, usize, usize, usize, usize)> {
        let mut b = Arr2d::with_gen((t.d1() + 1) / 2, t.d2() / 2, |i, j| {
            t[(2 * i, 2 * j + 1)] != b'.'
        });
        let mut res = Vec::new();
        for i in 0..b.d2() {
            let mut rows = Vec::new();
            for j in 0..b.d1() {
                if !b[(j, i)] {
                    continue;
                }
                let mut len = 0;
                while i + len < b.d2() && b[(j, i + len)] {
                    len += 1;
                }
                rows.push((len, j));
            }
            // res += rows.len().upper_div(2);
            rows.sort();
            rows.reverse();
            let mut single = Vec::new();
            let mut double = Vec::new();
            let mut skip = false;
            for j in rows.indices() {
                if skip {
                    skip = false;
                    continue;
                }
                if j + 1 < rows.len() && rows[j].0 == rows[j + 1].0 {
                    double.push(rows[j]);
                    double.push(rows[j + 1]);
                    skip = true;
                } else {
                    single.push(rows[j]);
                }
            }
            double.extend(single);
            rows = double;
            for j in rows.indices().step_by(2) {
                let len = if j + 1 < rows.len() {
                    let from = rows[j].1.min(rows[j + 1].1);
                    let to = rows[j].1.max(rows[j + 1].1);
                    res.push((from, i, to - 1, i + rows[j + 1].0 - 1, 0));
                    rows[j + 1].0
                } else {
                    if rows[j].1 == 0 {
                        res.push((rows[j].1, i, rows[j].1, i + rows[j].0 - 1, 1));
                    } else {
                        res.push((rows[j].1 - 1, i, rows[j].1 - 1, i + rows[j].0 - 1, 2));
                    }
                    rows[j].0
                };
                for k in 0..len {
                    b[(rows[j].1, i + k)] = false;
                    if j + 1 < rows.len() {
                        b[(rows[j + 1].1, i + k)] = false;
                    }
                }
            }
        }
        res
    }
    let mut ans = Vec::new();
    for (r_min, c_min, r_max, c_max, tp) in calc(&t) {
        ans.push((
            r_min,
            r_max,
            c_min,
            c_max,
            match tp {
                0 => Str::from(b"TB"),
                1 => Str::from(b"T"),
                2 => Str::from(b"B"),
                _ => unreachable!(),
            },
        ));
    }
    for (c_min, r_min, c_max, r_max, tp) in calc(&t.transpose()) {
        ans.push((
            r_min,
            r_max,
            c_min,
            c_max,
            match tp {
                0 => Str::from(b"LR"),
                1 => Str::from(b"L"),
                2 => Str::from(b"R"),
                _ => unreachable!(),
            },
        ));
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
