use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int();
    let a = input.read_int_vec(n).sorted();

    type Mod = ModIntF;
    let qx = a.copy_count(x);
    let less = a.copy_filter(|&v| v < x).count();
    let more = a.copy_filter(|&v| v > x).count();
    let mut ans = Mod::zero();
    let mut sum_cur = Mod::zero();
    let c = Combinations::<Mod>::new(n + 1);
    for i in (1..=qx).rev() {
        sum_cur += c.c(qx, i);
        ans += sum_cur * c.c(less + more, less + i - 1);
        if i != 1 {
            ans += sum_cur * c.c(less + more, more + i - 1);
        }
    }
    let mut i = 0;
    let mut j = 0;
    while i + j < n {
        if a[i] == x || a[Back(j)] == x {
            break;
        }
        if a[i] + a[Back(j)] < 2 * x {
            i += 1;
            continue;
        }
        if a[i] + a[Back(j)] > 2 * x {
            j += 1;
            continue;
        }
        let mut qi = 1;
        while i + qi < n && a[i + qi] == a[i] {
            qi += 1;
        }
        let mut qj = 1;
        while j + qj < n && a[Back(j + qj)] == a[Back(j)] {
            qj += 1;
        }
        ans += c.c(i + qi + j + qj, i + qi);
        ans += c.c(i + j, i);
        ans -= c.c(i + qi + j, i + qi);
        ans -= c.c(i + j + qj, i);
        i += qi;
        j += qj;
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
