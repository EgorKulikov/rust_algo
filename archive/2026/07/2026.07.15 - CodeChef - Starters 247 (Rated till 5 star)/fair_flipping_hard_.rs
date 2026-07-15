use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use std::collections::{BTreeSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let zeroes = s.copy_count(b'0');
    let ones = n - zeroes;

    if zeroes < k || ones < k {
        out.print_line(s);
        out.print_line(0);
        return;
    }
    if 2 * k == n {
        let t = Str::from(Vec::with_gen(n, |i| b'1' + b'0' - s[i]));
        if s < t {
            out.print_line(s);
            out.print_line(0);
        } else {
            out.print_line(t);
            out.print_line(1);
        }
        return;
    }
    let t = Str::from(s.clone().unwrap().sorted());
    if s == t {
        out.print_line(s);
        out.print_line(0);
        return;
    }
    out.print_line(t);
    let mut bad = 0;
    // let mut good = 0;
    for i in 0..zeroes {
        if s[i] == b'1' {
            bad += 1;
        }
    }
    // if bad == k {
    //     out.print_line(1);
    //     return;
    // }
    let mut reachable = (0..=zeroes).collect::<BTreeSet<_>>();
    let mut res = vec![0; zeroes + 1];
    reachable.remove(&bad);
    let mut queue = VecDeque::from(vec![bad]);
    while let Some(val) = queue.pop_front() {
        if val == 0 {
            // out.print_line(res[val].max(2));
            out.print_line(res[val]);
            return;
        }
        let cur_bad = val;
        let bad_to_good_max = cur_bad.min(k);
        let bad_to_good_min = k - (ones - cur_bad).min(k);
        let cur_good = zeroes - val;
        let good_to_bad_max = cur_good.min(k);
        let good_to_bad_min = k - (zeroes - cur_good).min(k);

        let from = val + good_to_bad_min - bad_to_good_max;
        let to = val + good_to_bad_max - bad_to_good_min;
        while let Some(&pos) = reachable.ceil(&from) {
            if pos > to {
                break;
            }
            reachable.remove(&pos);
            res[pos] = res[val] + 1;
            queue.push_back(pos);
        }
    }
    unreachable!();
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
