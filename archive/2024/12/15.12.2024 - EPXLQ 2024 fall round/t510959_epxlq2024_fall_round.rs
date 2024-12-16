//{"name":"T510959 [EPXLQ2024 fall round] 风吹起了从前","group":"Luogu","url":"https://www.luogu.com.cn/problem/T510959?contestId=201084","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6\n5 6 abcab\n7 10 abcba\n4 3 abc\n3 6 ade\n2 4 cde\n2 abc\n4 abc\n5 abc\n6 a\n7 c\n8 ab\n","output":"0\n3\n9\n15\n4\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T510959EPXLQ2024FallRound"}}}

use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::extensions::replace_with::ReplaceWith;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashMap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn read_str(input: &mut Input) -> Vec<u8> {
    let mut res = Vec::new();
    loop {
        let next = input.get().unwrap();
        if next.is_ascii_whitespace() {
            return res;
        }
        res.push(next);
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut memories = Vec::new();
    for _ in 0..n {
        let r = input.read_int();
        let v = input.read_long();
        let s = read_str(input);
        memories.push((r, v, s));
    }

    struct SumPayload {
        key: i32,
        value: i64,
        sum: i64,
    }

    impl Payload for SumPayload {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.value + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }
    }

    impl OrdPayload for SumPayload {
        type Key = i32;

        fn key(&self) -> &Self::Key {
            &self.key
        }

        fn union(a: Self, b: Self) -> Self {
            SumPayload {
                key: a.key,
                value: a.value + b.value,
                sum: a.sum + b.sum,
            }
        }
    }

    #[derive(Default)]
    struct Node {
        treap: Tree<SumPayload>,
        children: HashMap<u8, Box<Node>>,
    }

    impl Node {
        fn add(&mut self, s: &[u8], r: i32, v: i64) {
            self.treap.replace_with(|treap| {
                Tree::union(
                    treap,
                    Tree::new().do_with(|new_treap| {
                        new_treap.insert(SumPayload {
                            key: r,
                            value: v,
                            sum: v,
                        });
                    }),
                )
            });
            if !s.is_empty() {
                if !self.children.contains_key(&s[0]) {
                    self.children.insert(s[0], Box::new(Node::default()));
                }
                self.children.get_mut(&s[0]).unwrap().add(&s[1..], r, v);
            }
        }

        fn query(&mut self, s: &[u8], r: i32) -> i64 {
            if s.is_empty() {
                self.treap.range(..=&r).payload().map_or(0, |t| t.sum)
            } else {
                self.children.get_mut(&s[0]).unwrap().query(&s[1..], r)
            }
        }
    }

    let mut root = Node::default();
    for (r, v, s) in memories {
        root.add(s.as_slice(), r, v);
    }

    for _ in 0..m {
        let d = input.read_int();
        let prefix = read_str(input);
        out.print_line(root.query(prefix.as_slice(), d));
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
