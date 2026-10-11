use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let u = input.read_size() - 1;

    if n == 1 {
        out.print_line((b'!', 0));
        return;
    }

    let mut other = Vec::with_capacity(n - 1);
    for i in 0..n {
        if i != u {
            other.push(i);
        }
    }
    let mut s1 = Str::from(vec![b'0'; n]);
    let mut s2 = Str::from(vec![b'0'; n]);

    let mut left = 0;
    let mut right = n - 1;
    let mut is_equals = false;
    while left < right {
        let mid = (left + right + 1) / 2;
        s1.fill(b'0');
        s2.fill(b'0');
        s1[u] = b'1';
        s2[u] = b'1';
        for i in 0..mid {
            s1[other[i]] = b'1';
        }
        for i in mid..other.len() {
            s2[other[i]] = b'1';
        }
        out.print_line((b'?', &s1, &s2));
        out.flush();
        let res = input.read_char();
        match res {
            b'=' => {
                left = mid;
                right = mid;
                is_equals = true;
            }
            b'>' => {
                left = mid;
            }
            b'<' => {
                right = mid - 1;
            }
            _ => unreachable!(),
        }
    }
    let mut ans = Vec::new();
    s1.fill(b'0');
    s2.fill(b'0');
    s1[u] = b'1';
    s2[u] = b'1';
    for i in 0..left {
        s1[other[i]] = b'1';
    }
    for i in left..other.len() {
        s2[other[i]] = b'1';
    }
    for i in left..other.len() {
        s1[other[i]] = b'1';
        s2[other[i]] = b'0';
        out.print_line((b'?', &s1, &s2));
        out.flush();
        let res = input.read_char();
        if res == b'<' {
            ans.push(other[i]);
        }
        s1[other[i]] = b'0';
        s2[other[i]] = b'1';
    }
    if !is_equals {
        s1[other[left]] = b'1';
        s2[other[left]] = b'0';
    }
    for i in 0..left {
        s1[other[i]] = b'0';
        s2[other[i]] = b'1';
        out.print_line((b'?', &s1, &s2));
        out.flush();
        let res = input.read_char();
        if res == b'>' {
            ans.push(other[i]);
        }
        s1[other[i]] = b'1';
        s2[other[i]] = b'0';
    }
    out.print_line((b'!', ans.len(), ans.inc()));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
