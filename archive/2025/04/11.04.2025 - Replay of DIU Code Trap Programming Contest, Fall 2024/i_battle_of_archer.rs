//{"name":"I. Battle of Archer","group":"Toph","url":"https://toph.co/arena?contest=diu-code-trap-fall-2024-r#!/p/67f37555cf762cd9aec91ed3","interactive":false,"timeLimit":1000,"tests":[{"input":"6 4\n1 2 3 4 5 6\n1 4 1\n2 2 4\n1 5 2\n2 3 5\n","output":"10\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Clone, Default)]
    struct Node {
        val: usize,
        delta: usize,
        decay: usize,
        right: usize,
        size: usize,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val + right_val.val;
            self.right = right_val.right;
            self.size = left_val.size + right_val.size;
        }

        fn accumulate(&mut self, value: &Self) {
            let add_right = value.delta - (value.right - self.right) * value.decay;
            let add_left = add_right - (self.size - 1) * value.decay;
            self.val += (add_left + add_right) * self.size / 2;
            self.delta += add_right;
            self.decay += value.decay;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
            self.decay = 0;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        val: a[i],
        delta: 0,
        decay: 0,
        right: i,
        size: 1,
    });

    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let p = input.read_size() - 1;
                let x = input.read_size();
                st.update(
                    p + 1 - x..=p,
                    &Node {
                        val: 0,
                        delta: x,
                        decay: 1,
                        right: p,
                        size: 0,
                    },
                );
            }
            2 => {
                let a = input.read_size() - 1;
                let b = input.read_size();
                out.print_line(st.query(a..b).val);
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
