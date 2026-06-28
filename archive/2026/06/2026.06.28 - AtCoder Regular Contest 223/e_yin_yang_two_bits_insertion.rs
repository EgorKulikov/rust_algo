use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(m);

    if (m - n) % 2 == 1 {
        out.print_line(false);
        return;
    }
    if a[0] != b[0] {
        out.print_line(false);
        return;
    }
    let mut a_same = vec![0];
    for i in 0..n - 1 {
        if a[i] == a[i + 1] {
            a_same.push(i + 1);
        }
    }
    a_same.push(n);
    let mut b_same = vec![0];
    for i in 0..m - 1 {
        if b[i] == b[i + 1] {
            b_same.push(i + 1);
        }
    }
    b_same.push(m);

    if a_same.len() != b_same.len() {
        out.print_line(false);
        return;
    }
    let mut carry = 0;
    let mut add = 0;
    for i in 1..a_same.len() {
        let a_base = a_same[i] - a_same[i - 1];
        let a = a_base + carry;
        let b = b_same[i] - b_same[i - 1];
        if a > b {
            out.print_line(false);
            return;
        }
        if a == 1 {
            if add > 0 || b == 1 || b % 2 == 0 {
                carry = (b - a) % 2;
            } else {
                carry = (b - a) % 2 + 2;
            }
        } else {
            carry = (b - a) % 2;
        }
        add = b - a - carry;
        assert_eq!(add % 2, 0);
    }
    out.print_line(carry == 0);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
