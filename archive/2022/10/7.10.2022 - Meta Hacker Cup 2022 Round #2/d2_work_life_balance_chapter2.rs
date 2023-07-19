//{"name":"D2: Work-Life Balance - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 3\n1 2\n2 1 1\n1 2 1\n1 2 1\n4 3\n1 1 1 2\n1 2 2\n2 2 2\n4 1 2\n8 5\n1 1 1 1 2 2 2 2\n5 2 4\n7 2 3\n6 2 5\n1 2 4\n3 2 4\n","output":"Case #1: -2\nCase #2: 0\nCase #3: 16\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2WorkLifeBalanceChapter2"}}}

use algo_lib::collections::treap::{TreapNode, Updateable};
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::direction::Direction;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        m: usize,
        a: Vec<usize>,
        q: Vec<(usize, usize, usize)>,
        ans: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.m = input.read();
            self.a = input.read_vec(self.n).dec_by_one();
            self.q = input.read_vec(self.m);
        }

        fn solve(&mut self) {
            struct Payload {
                qty: usize,
                pos: usize,
                sum_pos: usize,
            }

            impl Updateable for Payload {
                fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
                    self.qty =
                        left.map(|x| x.qty).unwrap_or(0) + right.map(|x| x.qty).unwrap_or(0) + 1;
                    self.sum_pos = left.map(|x| x.sum_pos).unwrap_or(0)
                        + right.map(|x| x.sum_pos).unwrap_or(0)
                        + self.pos;
                }
            }

            let mut treaps = vec![TreapNode::NONE, TreapNode::NONE];
            for i in 0..self.n {
                let (mut left, right) = treaps[self.a[i]].split(&i, false);
                let mid = TreapNode::new(
                    i,
                    Payload {
                        qty: 1,
                        sum_pos: i,
                        pos: i,
                    },
                );
                left.merge(mid);
                left.merge(right);
                treaps[self.a[i]] = left;
            }

            for &(mut x, mut y, z) in &self.q {
                x -= 1;
                y -= 1;
                let (mut left, right) = treaps[self.a[x]].split(&x, true);
                let (mut left, _) = left.split(&x, false);
                left.merge(right);
                treaps[self.a[x]] = left;
                self.a[x] = y;
                let (mut left, right) = treaps[y].split(&x, false);
                let mid = TreapNode::new(
                    x,
                    Payload {
                        qty: 1,
                        sum_pos: x,
                        pos: x,
                    },
                );
                left.merge(mid);
                left.merge(right);
                treaps[y] = left;

                let (mut left_1, mut right_1) = treaps[0].split(&z, false);
                let (mut left_2, mut right_2) = treaps[1].split(&z, false);

                let lv = left_1.data().map(|p| p.qty).unwrap_or(0)
                    + left_2.data().map(|p| p.qty).unwrap_or(0) * 2;
                let rv = right_1.data().map(|p| p.qty).unwrap_or(0)
                    + right_2.data().map(|p| p.qty).unwrap_or(0) * 2;

                if lv % 2 != rv % 2 {
                    self.ans -= 1;
                    left_1.merge(right_1);
                    treaps[0] = left_1;
                    left_2.merge(right_2);
                    treaps[1] = left_2;
                    continue;
                }

                if lv == rv {
                    left_1.merge(right_1);
                    treaps[0] = left_1;
                    left_2.merge(right_2);
                    treaps[1] = left_2;
                    continue;
                }

                fn solve(
                    left1: &mut TreapNode<usize, Payload>,
                    right2: &mut TreapNode<usize, Payload>,
                    delta: usize,
                ) -> i64 {
                    let qty = left1
                        .data()
                        .map(|p| p.qty)
                        .unwrap_or(0)
                        .min(right2.data().map(|p| p.qty).unwrap_or(0));
                    if delta > 2 * qty {
                        return -1;
                    }
                    let need = delta / 2;
                    let mut to_right = 0;
                    let mut target = None;
                    left1.binary_search(|key, _, _, right_data| {
                        let rq = right_data.map(|p| p.qty).unwrap_or(0) + 1;
                        if to_right + rq == need {
                            target = Some(*key);
                            return None;
                        }
                        if to_right + rq < need {
                            to_right += rq;
                            Some(Direction::Left)
                        } else {
                            Some(Direction::Right)
                        }
                    });
                    let (mut ll1, ll2) = left1.split(&target.unwrap(), false);
                    let a = ll2.data().map(|p| p.sum_pos).unwrap_or(0);
                    ll1.merge(ll2);
                    *left1 = ll1;
                    let mut to_left = 0;
                    let mut target = None;
                    right2.binary_search(|key, _, left_data, _| {
                        let lq = left_data.map(|p| p.qty).unwrap_or(0) + 1;
                        if to_left + lq == need {
                            target = Some(*key);
                            return None;
                        }
                        if to_left + lq < need {
                            to_left += lq;
                            Some(Direction::Right)
                        } else {
                            Some(Direction::Left)
                        }
                    });
                    let (mut rr1, rr2) = right2.split(&target.unwrap(), true);
                    let b = rr1.data().map(|p| p.sum_pos).unwrap_or(0);
                    rr1.merge(rr2);
                    *right2 = rr1;
                    if a >= b {
                        (a - b).into_i64()
                    } else {
                        (b - a).into_i64()
                    }
                }

                if lv < rv {
                    self.ans += solve(&mut left_1, &mut right_2, rv - lv);
                } else {
                    self.ans += solve(&mut left_2, &mut right_1, lv - rv);
                }
                left_1.merge(right_1);
                treaps[0] = left_1;
                left_2.merge(right_2);
                treaps[1] = left_2;
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
