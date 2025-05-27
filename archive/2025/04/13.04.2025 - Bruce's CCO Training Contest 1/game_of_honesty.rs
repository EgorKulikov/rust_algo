//{"name":"Game of Honesty","group":"DMOJ","url":"https://dmoj.ca/problem/oly19practice43","interactive":false,"timeLimit":600,"tests":[{"input":"18 4\n2 10 6\n5 18 6\n3 11 9\n11 15 12\n","output":"3\n"},{"input":"5 3\n2 4 3\n1 4 1\n4 4 5\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_last_false;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let lrx = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut poi = vec![0, n];
    for (l, r, _) in lrx.copy_iter() {
        poi.push(l);
        poi.push(r + 1);
    }
    poi.sort_unstable();
    poi.dedup();
    #[derive(Clone)]
    struct Node {
        val: i32,
        delta: i32,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                val: i32::MAX,
                delta: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val)
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta.maxim(value.delta);
            self.val.maxim(value.delta);
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let ans = search_last_false(0, m, |x| {
        let mut st = SegmentTree::with_gen(poi.len() - 1, |_| Node { val: 0, delta: 0 });
        let mut segs_by_x = DefaultHashMap::new(Vec::new());
        for (l, r, x) in lrx.copy_take(x) {
            let l = poi.lower_bound(&l);
            let r = poi.lower_bound(&(r + 1));
            st.update(l..r, &Node { val: x, delta: x });
            segs_by_x[x].push((l, r - 1));
        }
        let mut vals_by_x = DefaultHashMap::new(Vec::new());
        for i in 0..poi.len() - 1 {
            let v = st.point_query(i).val;
            if v != 0 {
                vals_by_x[v].push(i);
            }
        }
        for (x, segs) in segs_by_x {
            let mut l = 0;
            let mut r = poi.len() - 2;
            for (a, b) in segs {
                l.maxim(a);
                r.minim(b);
            }
            if l > r {
                return true;
            }
            let mut found = false;
            for i in vals_by_x[x].copy_iter() {
                if i >= l && i <= r {
                    found = true;
                    break;
                }
            }
            if !found {
                return true;
            }
        }
        false
    });
    out.print_line((ans + 1) % (m + 1));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
