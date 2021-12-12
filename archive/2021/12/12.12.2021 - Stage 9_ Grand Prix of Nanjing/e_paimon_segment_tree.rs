//{"name":"E. Paimon Segment Tree","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1 1\n8 1 6\n2 3 2\n2 2 0 0\n","output":"1\n"},{"input":"4 3 3\n2 3 2 2\n1 1 6\n1 3 3\n1 3 6\n2 2 2 3\n1 4 1 3\n4 4 2 3\n","output":"180\n825\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPaimonSegmentTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let q = input.read();
    type Mod = ModInt7;
    let a: Vec<Mod> = input.read_vec(n);
    let changes: Vec<(usize, usize, Mod)> = input.read_vec(m);
    let queries: Vec<(usize, usize, usize, usize)> = input.read_vec(q);

    #[derive(Debug)]
    struct Node {
        sum: Mod,
        sum_sq: Mod,
        qty: Mod,
        last_update: usize,
        answer: Mod,
        delta: Mod,
        sum_delta: Mod,
        delta_sq: Mod,
        sum_delta_sq: Mod,
    }

    impl Node {
        fn actualize(&mut self, update: usize) {
            let since = Mod::new((update - self.last_update) as i32);
            self.answer += since * self.sum_sq;
            self.sum_delta += since * self.delta;
            self.sum_delta_sq += since * self.delta_sq;
            self.last_update = update;
        }

        fn accumulate(&mut self, parent: &Self) {
            self.answer +=
                parent.sum_delta_sq * self.qty + Mod::new(2) * self.sum * parent.sum_delta;
            self.sum_delta_sq += Mod::new(2) * self.delta * parent.sum_delta + parent.sum_delta_sq;
            self.sum_delta += parent.sum_delta;
            self.add_delta(parent.delta);
        }

        fn add_delta(&mut self, delta: Mod) {
            self.sum_sq += delta * delta * self.qty + Mod::new(2) * self.sum * delta;
            self.sum += self.qty * delta;
            self.delta += delta;
            self.delta_sq = self.delta * self.delta;
        }

        fn new(left: usize, right: usize) -> Self {
            Node {
                sum: Mod::zero(),
                sum_sq: Mod::zero(),
                qty: Mod::new((right - left) as i32),
                last_update: 0,
                answer: Mod::zero(),
                delta: Mod::zero(),
                sum_delta: Mod::zero(),
                delta_sq: Mod::zero(),
                sum_delta_sq: Mod::zero(),
            }
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.sum = left.sum + right.sum;
            self.sum_sq = left.sum_sq + right.sum_sq;
            self.answer = left.answer + right.answer;
        }

        fn reset_delta(&mut self) {
            self.delta = Mod::zero();
            self.delta_sq = Mod::zero();
            self.sum_delta = Mod::zero();
            self.sum_delta_sq = Mod::zero();
        }
    }

    struct Tree {
        n: usize,
        nodes: Vec<Node>,
    }

    impl Tree {
        fn new(a: Vec<Mod>) -> Self {
            let mut res = Tree {
                n: a.len(),
                nodes: Vec::with_capacity(2 * a.len() - 1),
            };
            res.init(2 * a.len() - 2, 0, a.len(), &a);
            res
        }

        fn init(&mut self, root: usize, left: usize, right: usize, a: &Vec<Mod>) {
            if left + 1 == right {
                self.nodes.push(Node {
                    sum: a[left],
                    sum_sq: a[left] * a[left],
                    qty: Mod::one(),
                    last_update: 0,
                    answer: Mod::zero(),
                    delta: Mod::zero(),
                    sum_delta: Mod::zero(),
                    delta_sq: Mod::zero(),
                    sum_delta_sq: Mod::zero(),
                });
            } else {
                let mid = (left + right) >> 1;
                self.init(root - 2 * (right - mid), left, mid, a);
                self.init(root - 1, mid, right, a);
                let mut res = Node::new(left, right);
                res.join(&self.nodes[root - 2 * (right - mid)], &self.nodes[root - 1]);
                self.nodes.push(res);
            }
        }

        fn push_down(
            &mut self,
            root: usize,
            left_child: usize,
            right_child: usize,
            current: usize,
        ) {
            self.nodes[root].actualize(current);
            self.nodes[left_child].actualize(current);
            self.nodes[right_child].actualize(current);
            let (head, tail) = self.nodes.split_at_mut(root);
            head[left_child].accumulate(&tail[0]);
            head[right_child].accumulate(&tail[0]);
            self.nodes[root].reset_delta();
        }

        fn update(&mut self, from: usize, to: usize, current: usize, delta: Mod) {
            self.do_update(2 * self.n - 2, 0, self.n, from, to, current, delta);
        }

        fn do_update(
            &mut self,
            root: usize,
            left: usize,
            right: usize,
            from: usize,
            to: usize,
            current: usize,
            delta: Mod,
        ) {
            if to <= left || right <= from {
                return;
            }
            if from <= left && right <= to {
                self.nodes[root].actualize(current);
                self.nodes[root].add_delta(delta);
                // println!("{} {} {} {:#?}", left, right, current, self.nodes[root]);
                return;
            }
            let mid = (left + right) >> 1;
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.push_down(root, left_child, right_child, current);
            self.do_update(left_child, left, mid, from, to, current, delta);
            self.do_update(right_child, mid, right, from, to, current, delta);
            let (head, tail) = self.nodes.split_at_mut(root);
            tail[0].join(&head[left_child], &head[right_child]);
            // println!("{} {} {} {:#?}", left, right, current, self.nodes[root]);
        }

        fn query(&mut self, from: usize, to: usize, current: usize) -> Mod {
            self.do_query(2 * self.n - 2, 0, self.n, from, to, current)
        }

        fn do_query(
            &mut self,
            root: usize,
            left: usize,
            right: usize,
            from: usize,
            to: usize,
            current: usize,
        ) -> Mod {
            if to <= left || right <= from {
                return Mod::zero();
            }
            if from <= left && right <= to {
                self.nodes[root].actualize(current);
                // println!("{} {} {} {:#?}", left, right, current, self.nodes[root]);
                return self.nodes[root].answer;
            }
            let mid = (left + right) >> 1;
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.push_down(root, left_child, right_child, current);
            let mut res = self.do_query(left_child, left, mid, from, to, current);
            res += self.do_query(right_child, mid, right, from, to, current);
            let (head, tail) = self.nodes.split_at_mut(root);
            tail[0].join(&head[left_child], &head[right_child]);
            // println!("{} {} {} {:#?}", left, right, current, self.nodes[root]);
            res
        }
    }

    let mut tree = Tree::new(a);

    let mut ans = vec![Mod::zero(); q];
    let mut queries_at = vec![Vec::new(); m + 2];
    for (i, (_, _, x, y)) in queries.iter().enumerate() {
        queries_at[*x].push((i, -Mod::one()));
        queries_at[*y + 1].push((i, Mod::one()));
    }
    for i in 0..m + 2 {
        for (j, c) in queries_at[i].drain(..) {
            let query = tree.query(queries[j].0 - 1, queries[j].1, i);
            ans[j] += c * query;
        }
        if i > 0 && i <= m {
            tree.update(changes[i - 1].0 - 1, changes[i - 1].1, i, changes[i - 1].2);
        }
    }
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
