//{"name":"F1. Kevin and Binary String (Easy Version)","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/F1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n0001100111\n0000011111\n010101\n111000\n0101\n0110\n0101\n1010\n011001\n001110\n0\n1\n","output":"1\n3\n1\n-1\n-1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    #[derive(Eq, PartialEq)]
    struct Block {
        val: u8,
        count: usize,
    }

    fn build(s: Str) -> VecDeque<Block> {
        let mut res = VecDeque::new();
        let mut start = 0;
        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                res.push_back(Block {
                    val: s[start],
                    count: i - start,
                });
                start = i;
            }
        }
        res.push_back(Block {
            val: s[start],
            count: s.len() - start,
        });
        res
    }
    let mut s_blocks = build(s);
    let mut t_blocks = build(t);

    let mut ans = 0;
    while !s_blocks.is_empty() {
        let mut s_head = s_blocks.pop_front().unwrap();
        let t_head = t_blocks.pop_front().unwrap();
        if s_head == t_head {
            continue;
        }
        if s_head.val == t_head.val && s_head.count > t_head.count {
            out.print_line(-1);
            return;
        }
        if s_head.val != t_head.val {
            if let Some(s_1) = s_blocks.pop_front() {
                if let Some(s_2) = s_blocks.pop_front() {
                    s_head = Block {
                        val: s_head.val,
                        count: s_head.count + s_2.count,
                    }
                }
                s_blocks.push_front(s_head);
                s_blocks.push_front(s_1);
                t_blocks.push_front(t_head);
                ans += 1;
            } else {
                out.print_line(-1);
                return;
            }
        } else {
            let Some(mut s_1) = s_blocks.pop_front() else {
                out.print_line(-1);
                return;
            };
            let Some(s_2) = s_blocks.pop_front() else {
                out.print_line(-1);
                return;
            };
            s_head = Block {
                val: s_head.val,
                count: s_head.count + s_2.count,
            };
            if let Some(s_3) = s_blocks.pop_front() {
                s_1 = Block {
                    val: s_1.val,
                    count: s_1.count + s_3.count,
                }
            }
            s_blocks.push_front(s_1);
            s_blocks.push_front(s_head);
            t_blocks.push_front(t_head);
            ans += 1;
        }
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
//END MAIN
