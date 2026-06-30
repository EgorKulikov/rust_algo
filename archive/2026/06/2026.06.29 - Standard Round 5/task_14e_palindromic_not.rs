use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::{Str, StrReader};
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    if n <= 3 {
        let mut len = FxHashMap::default();
        len.insert(s.clone(), vec![]);
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(s) = queue.pop_front() {
            if s == t {
                out.print_line(true);
                let res = len.get(&s).unwrap();
                out.print_line(res.len());
                out.print_per_line(res);
                return;
            }
            let cur = len.get(&s).unwrap().clone();
            for i in 0..n {
                for j in 0..i {
                    if s[i] == s[j] {
                        let mut s = s.clone();
                        let mut cur = cur.clone();
                        cur.push((j + 1, i + 1));
                        for k in j..=i {
                            s[k] = b'0' + b'1' - s[k];
                        }
                        if len.insert(s.clone(), cur).is_none() {
                            queue.push_back(s);
                        }
                    }
                }
            }
        }
        out.print_line(false);
        return;
    }
    fn build_groups(s: &Str) -> Vec<usize> {
        let mut cur = s[0];
        let mut len = 0;
        let mut res = Vec::new();
        for c in s.copy_iter() {
            if c != cur {
                res.push(len);
                len = 0;
                cur = c;
            }
            len += 1;
        }
        res.push(len);
        res
    }
    fn convert(mut s: Str) -> Vec<(usize, usize)> {
        let g = build_groups(&s);
        let mut cur = g.len() / 2;
        let mut res = Vec::new();
        if g[cur] == 1 {
            if cur > 0 && g[cur - 1] > 1 {
                cur -= 1;
            } else if cur + 1 < g.len() && g[cur + 1] > 1 {
                cur += 1;
            } else {
                let total = g.copy_take(cur - 1).sum::<usize>();
                res.push((total + 1, total + 3));
                for i in total..total + 3 {
                    s[i] = b'0' + b'1' - s[i];
                }
                res.extend_from_slice(&convert(s));
                return res;
            }
        }
        let sums = g.partial_sums();
        let mut from = cur;
        let mut to = cur + 1;
        while from != 0 || to != g.len() {
            res.push((sums[from] + 1, sums[to]));
            if from == 0 {
                s[0] = b'0' + b'1' - s[0];
            }
            from = from.saturating_sub(1);
            to = (to + 1).min(g.len());
        }
        if s[0] == b'1' {
            res.push((1, s.len()));
        }
        res
    }
    let front = convert(s);
    let back = convert(t).reversed();
    out.print_line(true);
    out.print_line(front.len() + back.len());
    assert!(front.len() + back.len() <= n + 10);
    out.print_per_line(&front);
    out.print_per_line(&back);
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
