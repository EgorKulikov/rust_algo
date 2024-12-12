//{"name":"Assignment of Tasks","group":"CodeChef - START164A","url":"https://www.codechef.com/START164A/problems/ASSIGNTASKS","interactive":false,"timeLimit":6000,"tests":[{"input":"6\n1 1\n1\n1\n1\n3 2\n1 3 3\n1 3 3\n2 3\n3 2\n1 3 3\n1 3 3\n1 3\n5 3\n1 1 1 1 1\n1 1 1 1 1\n4 3 4\n6 4\n5 3 4 2 6 1\n1 2 3 4 5 6\n2 2 4 3\n1 5\n1\n1\n10 5 3 9 4\n","output":"2\n12\n9\n9\n25\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AssignmentOfTasks"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let s = input.read_size_vec(n);
    let f = input.read_size_vec(m);

    let mut left = 2;
    let mut right = n + 10 * n * n;

    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    struct Node {
        self_start: usize,
        id: usize,
        start: usize,
        end: usize,
        work: usize,
        self_work: usize,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.start = left.map_or(self.start, |l| l.start);
            self.work = self.self_work + left.map_or(0, |l| l.work) + right.map_or(0, |r| r.work);
            let mut end =
                left.map_or(self.self_start, |l| l.end).max(self.self_start) + self.self_work;
            if let Some(r) = right {
                end += r.work;
                end.maxim(r.end);
            }
            self.end = end;
        }
    }

    impl OrdPayload for Node {
        type Key = Self;

        fn key(&self) -> &Self::Key {
            self
        }
    }

    while left < right {
        let mid = (left + right) / 2;

        let mut treap = Tree::new();
        let mut at = 0;
        for i in 0..n {
            while at < m {
                treap.insert(Node {
                    self_start: s[i],
                    id: i,
                    start: s[i],
                    end: s[i] + a[i] * f[at],
                    work: a[i] * f[at],
                    self_work: a[i] * f[at],
                });
                if treap.payload().unwrap().end <= mid {
                    break;
                }
                at += 1;
                treap.detach();
            }
        }
        if at < m {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(left);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
