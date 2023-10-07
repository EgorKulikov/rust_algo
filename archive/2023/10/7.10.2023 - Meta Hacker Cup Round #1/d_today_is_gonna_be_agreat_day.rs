//{"name":"D: Today is Gonna be a Great Day","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-1/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"2\n3\n1 20 30\n2\n1 3\n1 2\n2\n1 1\n1\n1 2\n","output":"Case #1: 4\nCase #2: 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"today_is_gonna_be_a_great_day_.*input[.]txt"},"output":{"type":"file","fileName":"today_is_gonna_be_a_great_day_output.txt","pattern":null},"languages":{"java":{"taskClass":"DTodayIsGonnaBeAGreatDay"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;
use std::mem::swap;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        a: Vec<i32>,
        q: usize,
        queries: Vec<(usize, usize)>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.a = input.read_vec(self.n);
            self.q = input.read();
            self.queries = input.read_vec(self.q).dec();
        }

        fn solve(&mut self) {
            #[derive(Copy, Clone)]
            struct Node((i32, Reverse<usize>), (i32, Reverse<usize>), bool);

            impl SegmentTreeNode for Node {
                fn new(left: usize, _right: usize) -> Self {
                    Self((0, Reverse(left)), (0, Reverse(left)), false)
                }
                fn join(
                    &mut self,
                    left_val: &Self,
                    right_val: &Self,
                    _left: usize,
                    _mid: usize,
                    _right: usize,
                ) {
                    *self = Node(
                        left_val.0.max(right_val.0),
                        left_val.1.max(right_val.1),
                        false,
                    );
                }

                fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
                    if value.2 {
                        swap(&mut self.0, &mut self.1);
                        self.2 = !self.2;
                    }
                }

                fn reset_delta(&mut self, _left: usize, _right: usize) {
                    self.2 = false;
                }
            }

            impl Pushable<&()> for Node {
                fn push(&mut self, _delta: &(), _left: usize, _right: usize) {
                    swap(&mut self.0, &mut self.1);
                    self.2 = !self.2;
                }
            }

            let mut tree = SegmentTree::from_generator(self.n, |i| {
                Node(
                    (self.a[i], Reverse(i)),
                    (1000000007 - self.a[i], Reverse(i)),
                    false,
                )
            });
            for &(l, r) in self.queries.iter() {
                tree.update(l..=r, &());
                self.ans += tree.query(..).0 .1 .0 + 1;
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
        }
    }

    run_parallel::<Job>(input, output);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    solve(&mut input, &mut output, &pre_calc);
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
