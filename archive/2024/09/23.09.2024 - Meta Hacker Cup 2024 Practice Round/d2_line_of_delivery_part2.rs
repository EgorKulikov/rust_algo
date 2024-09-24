//{"name":"D2: Line of Delivery (Part 2)","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/practice-round/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n2 5\n7\n2\n3 6\n9\n5\n7\n4 8\n8\n7\n9\n6\n2 10\n15\n5\n4 4\n8\n5\n8\n5\n","output":"Case #1: 1 2\nCase #2: 2 2\nCase #3: 2 0\nCase #4: 1 5\nCase #5: 4 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"line_of_delivery_part_2_.*input[.]txt"},"output":{"type":"file","fileName":"line_of_delivery_part_2_output.txt","pattern":null},"languages":{"java":{"taskClass":"D2LineOfDeliveryPart2"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::treap::{Payload, PayloadWithKey, TreapNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use std::ops::DerefMut;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        g: usize,
        e: Vec<usize>,
        id: usize,
        dist: Option<usize>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.g = input.read();
            self.e = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            #[derive(Copy, Clone, Default)]
            struct Node {
                size: usize,
                pos: usize,
                delta: usize,
            }

            impl Payload for Node {
                fn reset_delta(&mut self) {
                    self.delta = 0;
                }

                fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
                    self.size = 1 + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
                }

                fn push_delta(&mut self, delta: &Self, _direction: Direction) {
                    self.delta += delta.delta;
                    self.pos -= delta.delta;
                }
            }

            impl PayloadWithKey for Node {
                type Key = usize;

                fn key(&self) -> &Self::Key {
                    &self.pos
                }
            }

            let mut root: TreapNode<Node> = TreapNode::NONE;
            for &pos in &self.e {
                let mut add = 0;
                root.binary_search_mut(|cur, left, _| {
                    let left_size = left.map_or(0, |l| l.size);
                    if add + left_size + pos >= cur.pos {
                        add += left_size + 1;
                        Some(Direction::Right)
                    } else {
                        Some(Direction::Left)
                    }
                });
                let (mut left, right) = root.split(&(add + pos), false);
                assert_eq!(left.payload().map_or(0, |data| data.size), add);
                left.push(
                    &Node {
                        delta: 1,
                        ..Default::default()
                    },
                    Direction::Left,
                );
                let mid = TreapNode::new(Node {
                    size: 1,
                    pos: add + pos,
                    delta: 0,
                });
                root = TreapNode::merge(left, TreapNode::merge(mid, right));
                /*let mut pos = Vec::with_capacity(self.n);
                let mut rec =
                    RecursiveFunction2::new(|rec, node: &TreapNode<Node>, delta: usize| {
                        if let Some(node) = node.deref() {
                            // node.push_down();
                            rec.call(node.left.deref(), delta + node.payload.delta);
                            pos.push(node.payload.pos - delta);
                            rec.call(node.right.deref(), delta + node.payload.delta);
                        }
                    });
                rec.call(&root, 0);
                eprintln!("{:?}", pos);*/
            }
            let mut pos = Vec::with_capacity(self.n);
            let mut rec = RecursiveFunction::new(|rec, node: &mut TreapNode<Node>| {
                if let Some(node) = node.deref_mut() {
                    node.push_down();
                    rec.call(node.left.deref_mut());
                    pos.push(node.payload.pos);
                    rec.call(node.right.deref_mut());
                }
            });
            rec.call(&mut root);
            for i in pos.indices().rev() {
                if self.dist.minim(pos[i].abs_diff(self.g)) {
                    self.id = self.n - i;
                }
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.id, self.dist));
        }
    }

    run_parallel::<Job>(input, output, false);
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

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
}
//END MAIN
