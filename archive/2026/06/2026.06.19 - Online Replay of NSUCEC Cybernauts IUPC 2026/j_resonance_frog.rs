use algo_lib::collections::default_map::{by_index, DefaultHashMap};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let pos = by_index(&b);
    let mut cur = 0;
    let mut delta = DefaultHashMap::new(0);
    for (k, v) in pos.iter() {
        delta[*k] = cur;
        cur += v.len();
    }
    #[derive(Clone)]
    struct Node {
        val: usize,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }
    }
    impl Default for Node {
        fn default() -> Self {
            Self {
                val: usize::MAX - 1,
            }
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    let mut res = vec![0; n];
    let d = delta[b[n - 1]];
    st.point_update(d + pos[b[n - 1]].len() - 1, Node { val: 0 });
    for i in (0..n - 1).rev() {
        let mut cand = res[i + 1] + 1;
        let pp = pos[a[i]].lower_bound(&i);
        if pp > 0 {
            let at = pos[a[i]][pp - 1];
            let lim = 2 * i - at;
            if lim < n {
                cand.minim(res[lim] + 1);
            }
            let qq = pos[a[i]].lower_bound(&lim);
            if qq > pp {
                let d = delta[a[i]];
                cand.minim(st.query(d + pp..d + qq).val + 1);
            }
        }
        res[i] = cand;
        let pos = pos[b[i]].lower_bound(&i);
        let d = delta[b[i]];
        st.point_update(d + pos, Node { val: cand });
    }
    out.print_line(res[0]);
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
