//{"name":"S4 - Magic Library","group":"DMOJ - OTHS Coding Competition 3 (Mock CCC)","url":"https://dmoj.ca/problem/othscc3p4","interactive":false,"timeLimit":3000,"tests":[{"input":"6 5\n1 4 3 4 6 6\n2 2 4 4\n1 2 4 1\n2 1 6 1\n1 4 5 5\n2 1 6 6\n","output":"2\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();

    enum Content {
        Sparse(Vec<i16>),
        Dense(Vec<i32>),
    }
    impl Default for Content {
        fn default() -> Self {
            Self::Sparse(Vec::new())
        }
    }
    #[derive(Default)]
    struct Node {
        content: Content,
        delta: Option<usize>,
        len: usize,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            match &mut self.content {
                Content::Sparse(s) => {
                    s.clear();
                    match &left_val.content {
                        Content::Sparse(t) => {
                            s.extend_from_slice(t);
                        }
                        Content::Dense(_) => unreachable!(),
                    }
                    match &right_val.content {
                        Content::Sparse(t) => {
                            s.extend_from_slice(t);
                        }
                        Content::Dense(_) => unreachable!(),
                    }
                }
                Content::Dense(q) => {
                    q.fill(0);
                    match &left_val.content {
                        Content::Sparse(t) => {
                            for &x in t {
                                q[x as usize] += 1;
                            }
                        }
                        Content::Dense(t) => {
                            for i in t.indices() {
                                q[i] += t[i];
                            }
                        }
                    }
                    match &right_val.content {
                        Content::Sparse(t) => {
                            for &x in t {
                                q[x as usize] += 1;
                            }
                        }
                        Content::Dense(t) => {
                            for i in t.indices() {
                                q[i] += t[i];
                            }
                        }
                    }
                }
            }
        }

        fn accumulate(&mut self, value: &Self) {
            if let Some(delta) = value.delta {
                self.delta = Some(delta);
                match &mut self.content {
                    Content::Sparse(s) => {
                        s.fill(delta as i16);
                    }
                    Content::Dense(q) => {
                        q.fill(0);
                        q[delta] = self.len as i32;
                    }
                }
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::with_gen_full(n, |l, r| Node {
        content: if r - l > 500 {
            Content::Dense(vec![0; 500])
        } else {
            Content::Sparse(a[l..r].iter().map(|x| *x as i16).collect::<Vec<_>>())
        },
        len: r - l,
        delta: None,
    });

    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let v = input.read_size() - 1;
                st.update(
                    l..r,
                    &Node {
                        content: Content::Sparse(Vec::new()),
                        delta: Some(v),
                        len: r - l,
                    },
                );
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let v = input.read_size() - 1;
                out.print_line(st.for_each(l..r, |res: usize, node| match &node.content {
                    Content::Sparse(s) => res + s.copy_count(v as i16),
                    Content::Dense(q) => res + q[v] as usize,
                }));
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
