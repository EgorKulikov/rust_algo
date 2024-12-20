//{"name":"F. MEX OR Mania","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n6 3\n0 0 1 0 1 0\n6 1\n3 2\n6 3\n3 1\n1 3 1\n1 1\n","output":"6\n3\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMEXORMania"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::default::default_vec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_size_vec(n);

    struct Node {
        kind: usize,
        qty: Vec<u32>,
        good: usize,
        from: usize,
        to: usize,
    }

    impl Node {
        fn new(kind: usize, from: usize, to: usize) -> Self {
            Self {
                kind,
                qty: vec![0; 1 << kind],
                good: 1 << kind,
                from,
                to,
            }
        }

        fn add(&mut self, x: usize) {
            self.qty[x] += 1;
            if self.qty[x] == 1 {
                self.good -= 1;
            }
        }

        fn remove(&mut self, x: usize) {
            self.qty[x] -= 1;
            if self.qty[x] == 0 {
                self.good += 1;
            }
        }

        fn len(&self) -> usize {
            self.to - self.from + 1
        }

        fn split(mut self, pos: usize, a: &[usize]) -> Vec<Node> {
            if pos - self.from < (1 << self.kind) {
                if self.to - pos < (1 << self.kind) {
                    vec![]
                } else {
                    for i in self.from..pos {
                        self.remove(a[i]);
                    }
                    self.from = pos + 1;
                    vec![self]
                }
            } else {
                if self.to - pos < (1 << self.kind) {
                    for i in pos + 1..=self.to {
                        self.remove(a[i]);
                    }
                    self.to = pos - 1;
                    vec![self]
                } else {
                    if self.to - pos < pos - self.from {
                        let mut other = Node::new(self.kind, pos + 1, self.to);
                        for i in pos + 1..=self.to {
                            other.add(a[i]);
                            self.remove(a[i]);
                        }
                        self.to = pos - 1;
                        vec![self, other]
                    } else {
                        let mut other = Node::new(self.kind, self.from, pos - 1);
                        for i in self.from..pos {
                            other.add(a[i]);
                            self.remove(a[i]);
                        }
                        self.from = pos + 1;
                        vec![self, other]
                    }
                }
            }
        }
    }
    let mut nodes = Vec::new();
    let mut sets = Vec::new();
    let mut answers = Vec::new();
    for i in 0.. {
        if n < (1 << i) {
            break;
        }
        let threshold = 1 << i;
        let mut start = 0;
        let mut cur_nodes = default_vec(n);
        let mut cur_set = BTreeSet::new();
        let mut cur_answer = BTreeSet::new();
        for j in 0..=n {
            if j == n || a[j] >= threshold {
                if j - start >= threshold {
                    let mut node = Node::new(i, start, j - 1);
                    for k in start..j {
                        node.add(a[k]);
                    }
                    if node.good == 0 {
                        cur_answer.insert((Reverse(node.len()), start));
                    }
                    cur_nodes[start] = Some(node);
                    cur_set.insert(start);
                }
                start = j + 1;
            }
        }
        nodes.push(cur_nodes);
        sets.push(cur_set);
        answers.push(cur_answer);
    }

    for _ in 0..q {
        let pos = input.read_size() - 1;
        let x = input.read_size();
        let mut ans = 0;
        for i in nodes.indices() {
            if a[pos] < (1 << i) {
                if let Some(&id) = sets[i].floor(&pos) {
                    if nodes[i][id].as_ref().unwrap().to >= pos {
                        if nodes[i][id].as_ref().unwrap().good == 0 {
                            answers[i].remove(&(Reverse(nodes[i][id].as_ref().unwrap().len()), id));
                        }
                        nodes[i][id].as_mut().unwrap().remove(a[pos]);
                        if a[pos] + x < (1 << i) {
                            nodes[i][id].as_mut().unwrap().add(a[pos] + x);
                            if nodes[i][id].as_ref().unwrap().good == 0 {
                                answers[i]
                                    .insert((Reverse(nodes[i][id].as_ref().unwrap().len()), id));
                            }
                        } else {
                            let mut new_nodes = nodes[i][id].take().unwrap().split(pos, &a);
                            sets[i].remove(&id);
                            for node in new_nodes.drain(..) {
                                if node.good == 0 {
                                    answers[i].insert((Reverse(node.len()), node.from));
                                }
                                let from = node.from;
                                nodes[i][from] = Some(node);
                                sets[i].insert(from);
                            }
                        }
                    }
                }
            }
            if let Some(&(Reverse(len), _)) = answers[i].iter().next() {
                ans.maxim(len);
            }
        }
        a[pos] += x;
        out.print_line(ans);
    }
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
