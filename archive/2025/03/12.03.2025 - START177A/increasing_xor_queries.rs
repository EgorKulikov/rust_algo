//{"name":"Increasing XOR Queries","group":"CodeChef - START177A","url":"https://www.codechef.com/START177A/problems/INCXORQUERY","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 5\n1 2 5 3\n1 3\n1 4\n2 4\n3 4\n1 1\n5 4\n1 2 3 4 8\n1 5\n3 5\n2 3\n4 5\n1 2\n900\n1 1\n1 1\n3 3\n5 0 1\n1 2\n2 3\n1 3\n","output":"10011\n0101\n11\n110\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_unsigned_vec(n);

    #[derive(Default)]
    struct Node {
        xor: u32,
        forbidden: u32,
        need: u32,
    }

    impl Node {
        fn new(a: u32) -> Self {
            Self {
                xor: a,
                forbidden: if a == 0 { 0 } else { 1 << a.highest_bit() },
                need: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.xor = left_val.xor ^ right_val.xor;
            self.forbidden = left_val.forbidden
                | (right_val.forbidden & (!left_val.xor))
                | (right_val.need & left_val.xor);
            self.need = left_val.need
                | (right_val.need & (!left_val.xor))
                | (right_val.forbidden & left_val.xor);
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node::new(a[i]));

    let mut ans = Str::new();

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let mut has = 0;
        let res = st.for_each_with_init(
            l..r,
            |res, node| {
                if !res {
                    return false;
                }
                if (has & node.forbidden) != 0 {
                    return false;
                }
                if (has & node.need) != node.need {
                    return false;
                }
                has ^= node.xor;
                true
            },
            true,
        );
        if res {
            ans.push(b'1');
        } else {
            ans.push(b'0');
        }
    }
    out.print_line(ans);
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
