use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::suffix_array::SuffixArray;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut s = input.read_str();

    let t = s.clone();
    s += &t;
    let sa = SuffixArray::new(&s);
    let mut order = Vec::new();
    for i in 0..=2 * n {
        if sa[i] < n {
            order.push((sa[i], i));
        }
    }
    let mut ans = Arr2d::new(n, n, 0);
    for i in 0..n {
        for j in 1..n {
            let (prev, pos_prev) = order[j - 1];
            let (cur, pos_cur) = order[j];
            if sa.lcp(pos_prev, pos_cur) > i {
                ans[(i, cur)] = ans[(i, prev)];
            } else {
                ans[(i, cur)] = j;
            }
        }
    }
    for i in 1..n {
        for j in 0..n {
            ans[(i, j)] += ans[(i - 1, j)];
        }
    }
    let mut at = 0;
    for _ in 0..q {
        let a = input.read_size();
        at = (a + at) % n;
        let l = input.read_size() - 1;
        out.print_line(ans[(l, at)]);
    }
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
