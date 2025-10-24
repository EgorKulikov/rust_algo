//{"name":"P4 - Mathemagical Experiment","group":"DMOJ - OTHS Coding Competition 4","url":"https://dmoj.ca/problem/othscc4p4","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 3 4 2\n1 3 5\n2 4\n3 3\n","output":"32\n"},{"input":"4 4\n1 1 1 1\n2 -3\n3 1\n2 0\n3 2\n","output":"999999997\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    value!(Modulo: u32 = 1_000_000_000);
    type Mod = ModInt<Modulo>;

    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_vec::<Mod>(n);

    #[derive(Clone)]
    struct Node {
        mult: Mod,
        add: Mod,
    }

    impl Default for Node {
        fn default() -> Self {
            Node {
                mult: Mod::one(),
                add: Mod::zero(),
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.mult *= value.mult;
            self.add = self.add * value.mult + value.add;
        }

        fn reset_delta(&mut self) {
            self.mult = Mod::one();
            self.add = Mod::zero();
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        mult: Mod::one(),
        add: a[i],
    });
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let q = input.read_size() - 1;
                let v = input.read::<Mod>();
                st.for_each_mut(q..=q, |_, node| {
                    node.add += v;
                });
            }
            2 => {
                let m = input.read::<Mod>();
                st.for_each_mut(.., |_, node| {
                    node.mult *= m;
                });
            }
            3 => {
                let q = input.read_size() - 1;
                out.print_line(st.point_query(q).add);
            }
            _ => unreachable!(),
        }
    }
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
