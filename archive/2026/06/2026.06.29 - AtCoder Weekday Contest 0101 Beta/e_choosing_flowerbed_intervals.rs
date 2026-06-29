use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_int();
    let a = input.read_size_vec(n);
    let b = input.read_int_vec(n);

    struct Node {
        min: i32,
        max: i32,
        add: usize,
        len: usize,
    }
    impl Default for Node {
        fn default() -> Self {
            Self {
                min: i32::MAX,
                max: i32::MIN,
                add: 0,
                len: 0,
            }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.min = left_val.min.min(right_val.min);
            self.max = left_val.max.max(right_val.max);
            self.add = left_val.add + right_val.add;
            self.len = left_val.len + right_val.len;
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    let mut pos = FxHashMap::default();
    let mut ans = 0;
    for i in (0..n).rev() {
        if let Some(&pos) = pos.get(&a[i]) {
            st.point_update(
                pos,
                Node {
                    add: 0,
                    min: b[pos],
                    max: b[pos],
                    len: 1,
                },
            );
        }
        st.point_update(
            i,
            Node {
                add: 1,
                min: b[i],
                max: b[i],
                len: 1,
            },
        );
        pos.insert(a[i], i);
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut d = 0;
        let mut len = 0;
        st.binary_search_in(
            i..,
            |node| {
                if max.max(node.max) - min.min(node.min) <= m
                    && (d + node.add) * (len + node.len) <= k
                {
                    max.maxim(node.max);
                    min.minim(node.min);
                    d += node.add;
                    len += node.len;
                    false
                } else {
                    true
                }
            },
            |_, _| (),
        );
        ans += len;
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
