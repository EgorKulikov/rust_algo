//{"name":"D2. Inversion Graph Coloring (Hard Version)","group":"Codeforces - Codeforces Round 1051 (Div. 2)","url":"https://codeforces.com/contest/2143/problem/D2","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4\n4 2 3 1\n7\n7 6 1 2 3 3 2\n5\n1 1 1 1 1\n11\n7 2 1 9 7 3 4 1 3 5 3\n","output":"13\n73\n32\n619\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    type Mod = ModInt7;
    #[derive(Default)]
    struct Node {
        delta: Mod,
    }

    impl SegmentTreeNode for Node {
        fn reset_delta(&mut self) {
            self.delta = Mod::zero();
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
        }
    }

    let mut horizontal = Vec::with_gen(n, |_| {
        SegmentTree::with_gen(n, |_| Node { delta: Mod::one() })
    });
    let mut vertical = Vec::with_gen(n, |_| {
        SegmentTree::with_gen(n, |_| Node { delta: Mod::zero() })
    });

    for i in (0..n).rev() {
        for x in a[i] + 1..n {
            let val = horizontal[x].for_each(a[i]..=a[i], |_, node| node.delta)
                + vertical[a[i]].for_each(x..=x, |_, node| node.delta);
            horizontal[x].update(0..=a[i], &Node { delta: val });
        }
        for y in 0..=a[i] {
            let val = horizontal[a[i]].for_each(y..=y, |_, node| node.delta)
                + vertical[y].for_each(a[i]..=a[i], |_, node| node.delta);
            vertical[y].update(y..=a[i], &Node { delta: val });
        }
    }
    let val = horizontal[0].for_each(0..=0, |_, node| node.delta)
        + vertical[0].for_each(0..=0, |_, node| node.delta);
    out.print_line(val);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
