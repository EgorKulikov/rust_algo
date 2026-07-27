use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let q = p.inv();
    if n == 1 {
        out.print_line(1);
        return;
    }
    #[derive(Default, Clone)]
    struct Node {
        max: i32,
        delta: i32,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.max = left_val.max.max(right_val.max);
        }

        fn accumulate(&mut self, value: &Self) {
            self.max += value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::<Node>::new(2 * n);
    let mut parts = 0;
    let mut set = BitSet::new(n);
    for i in 0..n {
        parts += 1;
        if p[i] > 0 && set[p[i] - 1] {
            parts -= 1;
        }
        if p[i] + 1 < n && set[p[i] + 1] {
            parts -= 1;
        }
        set.set(p[i]);
        st.point_update(i, Node { max: parts, delta: 0 });
    }

    let mut ans = 0;
    for i in 0..n {
        if st.query(i..i + n).max <= 2 {
            ans += 1;
        }
        let mut pos = Vec::new();
        if p[i] > 0 {
            let mut cur = q[p[i] - 1];
            if cur < i {
                cur += n;
            }
            pos.push(cur);
        }
        if p[i] + 1 < n {
            let mut cur = q[p[i] + 1];
            if cur < i {
                cur += n;
            }
            pos.push(cur);
        }
        pos.push(i + 1);
        pos.push(i + n);
        pos.sort();
        let mut delta = -1;
        for (a, b) in pos.consecutive_iter_copy() {
            st.update(a..b, &Node { delta, ..Node::default() });
            delta += 1;
        }
        st.point_update(i + n, Node { max: 1, delta: 0 });
    }
    out.print_line(ans);
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
