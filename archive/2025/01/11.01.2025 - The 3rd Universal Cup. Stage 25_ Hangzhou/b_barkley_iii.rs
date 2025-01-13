//{"name":"B. Barkley III","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9727","interactive":false,"timeLimit":1000,"tests":[{"input":"5 9\n7 7 7 6 7\n3 1 5\n2 1 3\n3 1 5\n3 1 3\n1 1 2 3\n3 1 3\n2 2 8\n3 1 3\n3 1 2\n","output":"7\n6\n7\n3\n3\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBarkleyIII"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    // #[derive(Copy, Clone)]
    // enum State {
    //     None,
    //     One(usize),
    //     More,
    // }

    #[derive(Clone)]
    struct Node {
        and: i64,
        // state: Vec<State>,
        delta: i64,
        // single: State,
        cand: Vec<i64>,
        single: bool,
    }

    impl Node {
        fn single(a: i64) -> Self {
            // let mut state = Vec::with_capacity(63);
            // for j in 0..63 {
            //     if a.is_set(j) {
            //         state.push(State::None);
            //     } else {
            //         state.push(State::One(i));
            //     }
            // }
            // Node {
            //     and: a,
            //     state,
            //     delta: i64::MAX,
            //     single: State::One(i),
            // }
            Node {
                and: a,
                delta: i64::MAX,
                cand: vec![i64::MAX],
                single: true,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                and: i64::MAX,
                // state: vec![State::None; 63],
                delta: i64::MAX,
                // single: State::More,
                cand: Vec::new(),
                single: false,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.and = left_val.and & right_val.and;
            // for i in 0..63 {
            //     self.state[i] = match (left_val.state[i], right_val.state[i]) {
            //         (State::None, x) | (x, State::None) => x,
            //         _ => State::More,
            //     }
            // }
            self.cand.clear();
            for i in left_val.cand.copy_iter() {
                let may = i & right_val.and;
                if may > self.and {
                    self.cand.push(may);
                }
            }
            for i in right_val.cand.copy_iter() {
                let may = i & left_val.and;
                if may > self.and {
                    self.cand.push(may);
                }
            }
        }

        fn accumulate(&mut self, value: &Self) {
            if value.delta == i64::MAX {
                return;
            }
            self.and &= value.delta;
            self.delta &= value.delta;
            if !self.cand.is_empty() && !self.single {
                let mut i = 0;
                while i < self.cand.len() {
                    let n_val = self.cand[i] & value.delta;
                    if n_val > self.and {
                        self.cand[i] = n_val;
                        i += 1;
                    } else {
                        self.cand.swap_remove(i);
                    }
                }
            }
        }

        fn reset_delta(&mut self) {
            self.delta = i64::MAX;
        }
    }

    let mut st = SegmentTree::gen(n, |i| Node::single(a[i]));
    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        delta: x,
                        and: 0,
                        cand: Vec::new(),
                        single: false,
                    },
                );
            }
            2 => {
                let s = input.read_size() - 1;
                let x = input.read_long();
                st.point_update(s, Node::single(x));
            }
            3 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let node: Node = st.query(l..r);
                let ans = node.and.max(*node.cand.iter().max().unwrap_or(&0));
                out.print_line(ans);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
