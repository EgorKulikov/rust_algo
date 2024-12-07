//{"name":"F. Maximum modulo equality","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5 5\n5 14 2 6 3\n4 5\n1 4\n2 4\n3 5\n1 1\n1 1\n7\n1 1\n3 2\n1 7 8\n2 3\n1 2\n","output":"3 1 4 1 0\n0\n1 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaximumModuloEquality"}}}

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
    let a = input.read_int_vec(n);

    struct Node {
        value: i32,
        gcd_diff: i32,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.gcd_diff = 0;
            if let Some(left) = left {
                let res = gcd(self.gcd_diff, gcd(left.value - self.value, left.gcd_diff)).abs();
                self.gcd_diff = res;
            }
            if let Some(right) = right {
                let res = gcd(self.gcd_diff, gcd(right.value - self.value, right.gcd_diff)).abs();
                self.gcd_diff = res;
            }
        }
    }
    let mut treap = Treap::sized();
    for i in a {
        treap.add_back(Node {
            value: i,
            gcd_diff: 0,
        });
    }
    let mut ans = Vec::with_capacity(q);
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        ans.push(treap.by_index(l..r).payload().unwrap().gcd_diff);
    }
    out.print_line(ans);
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
