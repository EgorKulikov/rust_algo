//{"name":"F - Random Gathering","group":"AtCoder - AtCoder Beginner Contest 417","url":"https://atcoder.jp/contests/abc417/tasks/abc417_f","interactive":false,"timeLimit":3000,"tests":[{"input":"7 4\n30 10 40 10 50 90 20\n4 6\n5 7\n1 6\n3 7\n","output":"35 35 36 36 36 36 36\n"},{"input":"2 1\n0 1\n1 2\n","output":"499122177 499122177\n"},{"input":"15 10\n61477244 450343304 812961384 836482955 280670539 405068748 318805088 304825858 518212597 316347783 589272551 505875419 944071276 364842194 5376942\n2 11\n5 9\n8 15\n6 7\n6 8\n1 2\n1 10\n4 9\n12 15\n6 11\n","output":"449356308 449356308 449356308 449356308 449356308 648148154 648148154 648148154 648148154 648148154 648148154 643863031 643863031 643863031 643863031\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    type Mod = ModIntF;
    let a: Vec<Mod> = input.read_vec(n);
    let lr = input.read_size_pair_vec(m).dec();

    #[derive(Default, Clone)]
    struct Node {
        sum: Mod,
        len: Mod,
        delta: Option<Mod>,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum + right_val.sum;
        }

        fn accumulate(&mut self, value: &Self) {
            if let Some(delta) = value.delta {
                self.delta = Some(delta);
                self.sum = delta * self.len;
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let s = a.partial_sums();
    let mut st = SegmentTree::with_gen_full(n, |i, j| Node {
        sum: s[j] - s[i],
        len: Mod::from(j - i),
        delta: None,
    });

    for (l, r) in lr {
        let sum = st.query(l..=r).sum;
        st.update(
            l..=r,
            &Node {
                delta: Some(sum / Mod::from(r - l + 1)),
                ..Default::default()
            },
        );
    }
    out.print_line_iter((0..n).map(|i| st.query(i..=i).sum));
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
