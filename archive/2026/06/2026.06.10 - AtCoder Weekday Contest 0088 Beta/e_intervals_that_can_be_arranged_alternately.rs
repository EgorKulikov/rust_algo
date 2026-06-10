use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();
    let lr = input.read_size_pair_vec(q).dec();

    let mut delta = Vec::with_capacity(n + 1);
    delta.push(0);
    let mut cur = 0;
    for i in 0..n {
        if s[i] == b'W' {
            cur += 1;
        } else {
            cur -= 1;
        }
        delta.push(cur);
    }
    let mut queries = vec![Vec::new(); n + 1];
    for i in 0..q {
        let (l, r) = lr[i];
        queries[r + 1].push((l, i));
    }
    for i in 0..=n {
        queries[i].sort_by_key(|x| Reverse(x.0));
    }
    let mut ans = vec![0; q];
    const BUBEN: usize = 200;
    for i in (0..=n).step_by(BUBEN) {
        let mut cur = 0;
        let mut qty = DefaultHashMap::new(0usize);
        for j in i..=n {
            cur += qty[delta[j]] + qty[delta[j] - 1] + qty[delta[j] + 1];
            qty[delta[j]] += 1;
            while let Some(&(l, id)) = queries[j].last() {
                if l > i {
                    break;
                }
                queries[j].pop();
                let mut res = cur;
                for k in l..i {
                    res += qty[delta[k]] + qty[delta[k] - 1] + qty[delta[k] + 1];
                    qty[delta[k]] += 1;
                }
                ans[id] = res;
                for k in l..i {
                    qty[delta[k]] -= 1;
                }
            }
        }
    }
    for i in 0..=n {
        for (l, id) in queries[i].copy_iter() {
            let mut res = 0;
            let mut qty = DefaultHashMap::new(0usize);
            for k in l..=i {
                res += qty[delta[k]] + qty[delta[k] - 1] + qty[delta[k] + 1];
                qty[delta[k]] += 1;
            }
            ans[id] = res;
        }
    }
    out.print_per_line(&ans);
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
