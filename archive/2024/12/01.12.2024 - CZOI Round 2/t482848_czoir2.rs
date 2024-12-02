//{"name":"T482848 「CZOI-R2」天平","group":"Luogu","url":"https://www.luogu.com.cn/problem/T482848?contestId=203857","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 10 8 4 2\nI 2 1\nA 1 4 4\nA 2 4 4\nD 5\nQ 1 4 4\n","output":"YES\n"},{"input":"10 10\n2 2 1 4 2 10 8 7 10 6\nQ 5 6 1\nQ 5 7 7\nI 5 1\nQ 4 5 3\nQ 2 9 2\nA 3 5 1\nQ 7 8 5\nD 7\nA 3 9 7\nQ 3 7 6\n","output":"NO\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T482848CZOIR2"}}}

use algo_lib::collections::treap::{Payload, Treap};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    struct Node {
        weight: i64,
        delta: i64,
        gcd_diff: i64,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_PUSH_DOWN: bool = true;

        fn reset_delta(&mut self) {
            self.delta = 0;
        }

        fn push_delta(&mut self, delta: &Self) {
            self.weight += delta.delta;
            self.delta += delta.delta;
        }

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.gcd_diff = 0;
            if let Some(left) = left {
                self.gcd_diff = gcd(self.gcd_diff, gcd(left.gcd_diff, left.weight - self.weight));
            }
            if let Some(right) = right {
                self.gcd_diff = gcd(
                    self.gcd_diff,
                    gcd(right.gcd_diff, right.weight - self.weight),
                );
            }
        }

        fn need_push_down(&self) -> bool {
            self.delta != 0
        }
    }

    let mut treap = Treap::new();
    for i in 0..n {
        treap.add_back(Node {
            weight: a[i],
            delta: 0,
            gcd_diff: 0,
        });
    }

    for _ in 0..q {
        let op = input.read_char();
        match op {
            b'I' => {
                let x = input.read_size();
                let v = input.read_long();
                treap.index_range(x..x).add_back(Node {
                    weight: v,
                    delta: 0,
                    gcd_diff: 0,
                });
            }
            b'D' => {
                let x = input.read_size() - 1;
                treap.index_range(x..=x).detach();
            }
            b'A' => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let v = input.read_long();
                treap.index_range(l..r).push(&Node {
                    weight: 0,
                    delta: v,
                    gcd_diff: 0,
                });
            }
            b'Q' => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let v = input.read_long();
                out.print_line(
                    v % treap
                        .index_range(l..r)
                        .get_payload()
                        .map(|x| gcd(x.gcd_diff, x.weight))
                        .unwrap()
                        == 0,
                );
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
