//{"name":"S5 - To-Do List","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25s5","interactive":false,"timeLimit":4000,"tests":[{"input":"6\nA 3 3\nA 2 0\nA 999996 999995\nD 999991\nA 1000000 999994\nD 999992\n","output":"5\n11\n13\n11\n13\n9\n"},{"input":"6\nA 3 3\nA 7 5\nA 4 3\nD 1\nA 8 2\nD 2\n","output":"1999999\n2999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_size();

    struct Node {
        s: i64,
        t: i64,
        end: i64,
        sum: i64,
        key: (i64, usize),
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.t + left.map_or(0, |x| x.sum) + right.map_or(0, |x| x.sum);
            self.end = 0;
            if let Some(left) = left {
                self.end = left.end;
            }
            self.end = self.end.max(self.s) + self.t;
            if let Some(right) = right {
                self.end = (self.end + right.sum).max(right.end);
            }
        }
    }

    impl OrdPayload for Node {
        type Key = (i64, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let mut ans = 0;
    let mut next_add = 1;
    let mut tree = Tree::new();
    let mut ss = Vec::new();

    for _ in 0..q {
        let t = input.read_char();
        match t {
            b'A' => {
                let s = (input.read_long() + ans) % 1_000_003;
                let t = (input.read_long() + ans) % 1_000_003;
                tree.insert(Node {
                    s,
                    t,
                    end: s + t,
                    sum: t,
                    key: (s, next_add),
                });
                next_add += 1;
                ss.push(s);
            }
            b'D' => {
                let i = ((input.read_long() + ans) % 1_000_003) as usize;
                tree.remove(&(ss[i - 1], i));
            }
            _ => unreachable!(),
        }
        ans = tree.payload().unwrap().end - 1;
        out.print_line(ans);
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
