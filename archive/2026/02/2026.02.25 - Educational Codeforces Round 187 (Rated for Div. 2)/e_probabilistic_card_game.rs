//{"name":"E. Probabilistic Card Game","group":"Codeforces - Educational Codeforces Round 187 (Rated for Div. 2)","url":"https://codeforces.com/contest/2203/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1 10 3 11 7\n","output":"0\n499122177\n665496236\n"},{"input":"6\n23 7 11 24 10 28\n","output":"0\n499122177\n1\n748683266\n"},{"input":"9\n4 10 7 1 16 5 9 12 2\n","output":"0\n499122178\n2\n499122178\n798595484\n831870296\n427819010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    struct Node {
        val: i64,
        min: i64,
        max: i64,
        sum: i64,
        len: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |x| x.sum) + right.map_or(0, |x| x.sum);
            self.max = right.map_or(self.val, |x| x.max);
            self.min = left.map_or(self.val, |x| x.min);
            self.len = 1 + left.map_or(0, |x| x.len) + right.map_or(0, |x| x.len);
        }
    }
    impl OrdPayload for Node {
        type Key = i64;

        fn key(&self) -> &Self::Key {
            &self.val
        }
    }
    let mut tree = Tree::new();
    tree.insert(Node {
        val: a[0],
        sum: a[0],
        min: a[0],
        max: a[0],
        len: 1,
    });
    tree.insert(Node {
        val: a[1],
        sum: a[1],
        min: a[1],
        max: a[1],
        len: 1,
    });
    type Mod = ModIntF;
    for i in 2..n {
        tree.insert(Node {
            val: a[i],
            sum: a[i],
            min: a[i],
            max: a[i],
            len: 1,
        });
        let mut res = i64::MAX;
        let mut l_sum = 0;
        let mut l_qty = 0;
        let mut l_max = 0;
        let mut r_sum = 0;
        let mut r_qty = 0;
        let mut r_min = 0;
        tree.binary_search(|node, left, right| {
            let l_val = left.map_or(l_max * l_qty - l_sum, |x| {
                x.max * (x.len + l_qty) - x.sum - l_sum
            });
            let r_val = right.map_or(r_sum - r_min * r_qty, |x| {
                x.sum + r_sum - x.min * (x.len + r_qty)
            });
            res.minim(l_val.max(r_val));
            match l_val.cmp(&r_val) {
                std::cmp::Ordering::Less => {
                    l_sum += left.map_or(0, |x| x.sum) + node.val;
                    l_qty += left.map_or(0, |x| x.len) + 1;
                    l_max = node.val;
                    Some(Direction::Right)
                }
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => {
                    r_sum += right.map_or(0, |x| x.sum) + node.val;
                    r_qty += right.map_or(0, |x| x.len) + 1;
                    r_min = node.val;
                    Some(Direction::Left)
                }
            }
        });
        out.print_line(Mod::from(res) / (i - 1));
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
