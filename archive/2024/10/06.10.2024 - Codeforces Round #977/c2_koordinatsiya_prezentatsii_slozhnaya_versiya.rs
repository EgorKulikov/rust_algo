//{"name":"C2. Координация презентации (сложная версия)","group":"Codeforces - Codeforces Round 977 (Div. 2, на основе COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/C2","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 2 2\n1 2 3 4\n1 1\n1 2\n1 1\n3 6 2\n1 2 3\n1 1 2 3 3 2\n3 3\n2 2\n4 6 2\n3 1 4 2\n3 1 1 2 3 4\n3 4\n4 2\n","output":"YA\nTIDAK\nYA\nYA\nTIDAK\nYA\nTIDAK\nYA\nYA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2KoordinatsiyaPrezentatsiiSlozhnayaVersiya"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;
use std::iter::once;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let mut b = input.read_size_vec(m).dec();

    let pos = Permutation::new(a).inv().to_vec();
    #[derive(Copy, Clone)]
    struct Node {
        left: usize,
        right: usize,
        sorted: bool,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                left: usize::MAX,
                right: usize::MAX,
                sorted: true,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.left = left_val.left;
            self.right = right_val.right;
            self.sorted = left_val.sorted && right_val.sorted && left_val.right <= right_val.left;
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut vals = vec![once(m).collect::<BTreeSet<_>>(); n];
    for i in 0..m {
        let cur = pos[b[i]];
        vals[cur].insert(i);
    }
    let mut st = SegmentTree::from_generator(n, |i| Node {
        left: vals[i].iter().copied().next().unwrap(),
        right: vals[i].iter().copied().next().unwrap(),
        sorted: true,
    });
    out.print_line(st.query(..).sorted);
    for _ in 0..q {
        let s = input.read_size() - 1;
        let t = input.read_size() - 1;

        let was = pos[b[s]];
        vals[was].remove(&s);
        st.point_update(
            was,
            Node {
                left: vals[was].iter().copied().next().unwrap(),
                right: vals[was].iter().copied().next().unwrap(),
                sorted: true,
            },
        );
        b[s] = t;
        let now = pos[b[s]];
        vals[now].insert(s);
        st.point_update(
            now,
            Node {
                left: vals[now].iter().copied().next().unwrap(),
                right: vals[now].iter().copied().next().unwrap(),
                sorted: true,
            },
        );
        out.print_line(st.query(..).sorted);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::Custom("YA", "TIDAK"));

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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
