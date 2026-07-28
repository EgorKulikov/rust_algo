use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::{digits, sum_digs};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = Vec::new();
    for i in 1..=n {
        a.push(input.read_long_vec(i));
    }

    fn calc(v: i64) -> i64 {
        let r1 = sum_digs(v);
        let r2 = if v == 0 { 0 } else { digits(v).last().unwrap() - 1 + (digits(v).count() as i64 - 1) * 9 };
        r1.max(r2)
    }
    let d = Vec::with_gen(n, |i| a[i].copy_map(calc).collect::<Vec<_>>());

    #[derive(Default, Clone)]
    struct Node {
        val: i64,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
        }
    }
    let mut st = Vec::with_gen(n, |i| SegmentTree::with_gen(i + 1, |j| Node { val: d[i][j] }));

    for _ in 0..q {
        let l = input.read_size() - 1;
        let p = input.read_size() - 1;
        let t = input.read_size();
        if p > l {
            out.print_line("NA");
            continue;
        }
        let mut res = 0;
        for i in l..(l + t + 1).min(n) {
            res.maxim(st[i].query(p..=p + i - l).val);
        }
        out.print_line(res);
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
