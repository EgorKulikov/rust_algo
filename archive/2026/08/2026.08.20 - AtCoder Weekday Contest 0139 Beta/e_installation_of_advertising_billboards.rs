use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let l = input.read_size();
    let c = input.read_long();
    let r = input.read_long();
    let pd = input.read_vec::<(usize, i64)>(m).sorted();
    let q = input.read_size_vec(k).sorted();

    let mut poi = vec![l + 1, n + 1];
    for i in q.copy_iter() {
        if i > l {
            poi.push(i);
        }
        if i + l + 1 <= n {
            poi.push(i + l + 2);
        }
    }
    for (p, _) in pd.copy_iter() {
        if p > l {
            poi.push(p);
        }
        if p >= l {
            poi.push(p + 1);
        }
        if p + l <= n {
            poi.push(p + l + 1);
        }
        if p + l + 1 <= n {
            poi.push(p + l + 2);
        }
    }
    poi.sort();
    poi.dedup();
    let mut set_in = MultiTreeSet::new();
    let mut ans = None;
    let mut cur = 0;
    let mut shop_in = 0;
    let mut shop_out = 0;
    let mut q_in = 0;
    let mut q_out = 0;
    let mut total = 0;
    for p in poi {
        while shop_in < m && pd[shop_in].0 < p && pd[shop_in].0 >= cur {
            total += pd[shop_in].1;
            set_in.insert(pd[shop_in].1);
            shop_in += 1;
        }
        while shop_out < m && pd[shop_out].0 + l + 1 < p && pd[shop_out].0 + l + 1 >= cur {
            total -= pd[shop_out].1;
            set_in.remove(&pd[shop_out].1);
            shop_out += 1;
        }
        while q_in < k && q[q_in] < p && q[q_in] >= cur {
            total -= c;
            q_in += 1;
        }
        while q_out < k && q[q_out] + l + 1 < p && q[q_out] + l + 1 >= cur {
            total += c;
            q_out += 1;
        }
        let mut cand = total;
        if let Some(&v) = set_in.first() {
            if v < -r {
                cand += -r - v;
            }
        }
        ans.maxim(cand);
        cur = p;
    }
    out.print_line(ans);
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
