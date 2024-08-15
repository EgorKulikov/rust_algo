//{"name":"K Subsequence 101","group":"CodeChef - START147A","url":"https://www.codechef.com/START147A/problems/REMK","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5 3\n1 15 5 7 2\n4 2\n4 7 9 12\n6 6\n1 2 3 4 5 6\n","output":"38\n21\n35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KSubsequence101"}}}

use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let max = a.iter().max_position().unwrap();
    let mut ans = a[max];
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let mut inner = BinaryHeap::new();
    for i in 0..max {
        left.push((a[i], i));
    }
    for i in max + 1..n {
        right.push((a[i], i));
    }
    let mut from = max;
    let mut to = max;
    for _ in 1..k {
        let mut best = None;
        while let Some(&(_, id)) = left.peek() {
            if id >= from {
                left.pop();
            } else {
                break;
            }
        }
        if let Some(&(val, id)) = left.peek() {
            best.maxim((val + a[from], id));
        }
        while let Some(&(_, id)) = right.peek() {
            if id <= to {
                right.pop();
            } else {
                break;
            }
        }
        if let Some(&(val, id)) = right.peek() {
            best.maxim((val + a[to], id));
        }
        if let Some(&(val, id)) = inner.peek() {
            best.maxim((2 * val, id));
        }
        let (val, id) = best.unwrap();
        ans += val;
        if id < from {
            for i in id + 1..from {
                inner.push((a[i], i));
            }
            from = id;
        } else if id > to {
            for i in to + 1..id {
                inner.push((a[i], i));
            }
            to = id;
        } else {
            inner.pop();
        }
    }
    out.print_line(ans - a[max]);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
