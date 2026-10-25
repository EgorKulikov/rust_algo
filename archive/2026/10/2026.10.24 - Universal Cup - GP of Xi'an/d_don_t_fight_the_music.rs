use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_str();
    let mut b = input.read_str();

    if a[n - 1] == b'0' || a[0] == b'0' {
        out.print_line(-1);
        return;
    }
    if b[0] == b'0' || b[n - 1] == b'0' {
        out.print_line(-1);
        return;
    }
    a[0] = b'1';
    a[n - 1] = b'1';
    b[0] = b'1';
    b[n - 1] = b'1';
    fn fill_left(ans: &mut [usize], a: &Str, base: usize, to: usize) {
        let mut skipped = None;
        for i in 0..to {
            if a[i] == b'0' {
                if skipped.is_none() {
                    skipped = Some(i + base);
                }
                ans[i] = i + base + 1;
            } else {
                if let Some(v) = skipped {
                    skipped = None;
                    ans[i] = v;
                } else {
                    ans[i] = i + base;
                }
            }
        }
    }
    fn fill_right(ans: &mut [usize], b: &Str, base: usize, from: usize) {
        let mut skipped = None;
        let mut delta = 0;
        for i in (from..b.len()).rev() {
            if b[i] == b'0' {
                if skipped.is_none() {
                    skipped = Some(base - delta);
                }
                ans[i] = base - delta - 1;
            } else {
                if let Some(v) = skipped {
                    skipped = None;
                    ans[i] = v;
                } else {
                    ans[i] = base - delta;
                }
            }
            delta += 1;
        }
    }
    fn fill_middle(ans: &mut [usize], a: &Str, b: &Str, from: usize, to: usize) {
        let mut skipped = None;
        for i in from..to {
            if a[i] == b'0' || (i + 1) < a.len() && b[i + 1] == b'0' {
                if skipped.is_none() {
                    skipped = Some(i + 1);
                }
                ans[i] = i + 2;
            } else {
                if let Some(v) = skipped {
                    skipped = None;
                    ans[i] = v;
                } else {
                    ans[i] = i + 1;
                }
            }
        }
    }
    let mut rightmost10 = None;
    for i in (0..n - 1).rev() {
        if a[i] == b'1' && b[i + 1] == b'0' {
            rightmost10 = Some(i);
            break;
        }
    }
    let mut leftmost01 = None;
    for i in 0..n - 1 {
        if a[i] == b'0' && b[i + 1] == b'1' {
            leftmost01 = Some(i);
            break;
        }
    }
    if let Some(val) = rightmost10 {
        for j in 1..=val {
            if b[j] == b'1' {
                out.print_line(-1);
                return;
            }
            b[j] = b'0';
        }
    }
    if let Some(val) = leftmost01 {
        for j in val + 1..n - 1 {
            if a[j] == b'1' {
                out.print_line(-1);
                return;
            }
            a[j] = b'0';
        }
    }
    let mut ans = vec![0; n];
    match (rightmost10, leftmost01) {
        (Some(v1), Some(v2)) => {
            let mut no1 = true;
            let mut from = None;
            let mut to = None;
            for i in v1 + 1..v2 {
                if a[i] == b'1' || b[i + 1] == b'1' {
                    no1 = false;
                    if from.is_none() {
                        from = Some(i);
                    }
                    to = Some(i);
                }
            }
            if v1 + 1 == v2 {
                out.print_line(-1);
                return;
            } else if no1 {
                fill_left(&mut ans, &a, 2, v1 + 1);
                let mut top = n;
                for i in v1 + 1..v2 {
                    ans[i] = top;
                    top -= 1;
                }
                ans[v2] = 1;
                fill_right(&mut ans, &b, top, v2 + 1);
            } else {
                fill_left(&mut ans, &a, 1 + from.unwrap() - v1, v1 + 1);
                for i in v1 + 1..=from.unwrap() {
                    ans[i] = i - v1;
                }
                for i in to.unwrap() + 1..=v2 {
                    ans[i] = n - (v2 - i);
                }
                fill_right(&mut ans, &b, n - (v2 - to.unwrap()), v2 + 1);
                fill_middle(&mut ans, &a, &b, from.unwrap() + 1, to.unwrap() + 1);
            }
        }
        (Some(v1), None) => {
            let mut pos = n - 1;
            for i in v1 + 1..n - 1 {
                if a[i] != b'0' && b[i + 1] != b'0' {
                    pos = i;
                    break;
                }
            }
            for i in v1 + 1..=pos {
                ans[i] = i - v1;
            }
            fill_left(&mut ans, &a, 1 + pos - v1, v1 + 1);
            fill_middle(&mut ans, &a, &b, pos + 1, n);
        }
        (None, Some(v2)) => {
            let mut pos = 0;
            for i in (0..v2).rev() {
                if a[i] != b'0' && b[i + 1] != b'0' {
                    pos = i;
                    break;
                }
            }
            for i in pos + 1..=v2 {
                ans[i] = n - (v2 - i);
            }
            fill_right(&mut ans, &b, n - (v2 - pos), v2 + 1);
            fill_middle(&mut ans, &a, &b, 0, pos + 1);
        }
        (None, None) => {
            fill_middle(&mut ans, &a, &b, 0, n);
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
