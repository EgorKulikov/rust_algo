//{"name":"roman_numerals","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();
    let q = input.read_size();
    let pvd = input.read_vec::<(i32, i64, Str)>(m);
    let s = input.read_str_vec(n);
    let lr = input.read_size_pair_vec(q).dec();

    let mut id = FxHashMap::default();
    for i in 0..m {
        id.insert(pvd[i].2.clone(), i);
    }
    let s = Vec::with_gen(n, |i| *id.get(&s[i]).unwrap());
    let mut ans = vec![0i64; q];
    let mut r = vec![Vec::new(); n];
    for i in 0..q {
        r[lr[i].1].push((lr[i].0, i));
    }
    #[derive(Clone)]
    struct Node {
        val: i64,
        delta: i64,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { val: 0, delta: 1 }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val + right_val.val;
        }

        fn accumulate(&mut self, value: &Self) {
            self.val *= value.delta;
            self.delta *= value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 1;
        }
    }
    let mut st = SegmentTree::with_gen(n, |i| Node {
        val: pvd[s[i]].1,
        delta: 1,
    });
    let mut v = Vec::new();
    for i in 0..n {
        let cur_prio = pvd[s[i]].0;
        while let Some(&(pos, prio)) = v.last() {
            if prio >= cur_prio {
                break;
            }
            v.pop();
        }
        let from = if let Some(&(pos, _)) = v.last() {
            pos + 1
        } else {
            0
        };
        st.update(from..i, &Node { val: 0, delta: -1 });
        v.push((i, cur_prio));
        for &(l, idx) in &r[i] {
            ans[idx] = st.query(l..=i).val;
        }
    }
    out.print_per_line(&ans);
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
