use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();
    let mut b = input
        .iter_int()
        .take(n)
        .map(|x| {
            let res = if x == -1 { n } else { x as usize - 1 };
            res
        })
        .collect::<Vec<_>>();

    let mut free = BitSet::new(n);
    free.fill(true);
    for i in b.copy_iter() {
        if i != n {
            free.unset(i);
        }
    }
    let mut cycle_id = vec![n; n];
    let mut cur = 0;
    let mut free_cycles = vec![VecDeque::new(); n + 1];
    let mut cyc_len = Vec::new();
    for i in 0..n {
        if cycle_id[i] == n {
            let mut j = i;
            let mut all_free = true;
            let mut len = 0;
            while cycle_id[j] == n {
                len += 1;
                if !free[j] {
                    all_free = false;
                }
                cycle_id[j] = cur;
                j = a[j];
            }
            if all_free {
                free_cycles[len].push_back(i);
            }
            cyc_len.push(len);
            cur += 1;
        }
    }

    let mut processed = vec![0; n];
    let mut at = 0;
    for i in 0..n {
        if processed[i] == 2 {
            continue;
        }
        let mut expected = None;
        let mut j = i;
        let len = cyc_len[at];
        at += 1;
        while processed[j] < 2 {
            if processed[j] == 1 && expected.is_none() {
                if free_cycles[len].is_empty() {
                    out.print_line(false);
                    return;
                }
                expected = Some(free_cycles[len].pop_front().unwrap());
            }
            processed[j] += 1;
            if let Some(ex) = expected {
                if b[j] < n && b[j] != ex {
                    out.print_line(false);
                    return;
                }
                b[j] = ex;
                expected = Some(a[ex]);
            } else if b[j] != n {
                expected = Some(a[b[j]]);
            }
            j = a[j];
        }
    }
    for i in 0..=n {
        if !free_cycles[i].is_empty() {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
    out.print_line(b.inc());
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
