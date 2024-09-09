//{"name":"P4 - Candidates","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p4","interactive":false,"timeLimit":1000,"tests":[{"input":"11 3\n11 4 6 1 3 10 5 7 8 2 9\n4 4 2\n2 1 2\n2 4 2\n3 4 4\n4 3 4\n1 4 4\n2 3 3\n4 1 4\n1 1 2\n1 4 1\n3 4 4\n","output":"2 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4Candidates"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();
    let r = input.read_size_table(n, k);

    let mut bad = Vec::with_capacity(k);
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        let mut cur = HashSet::new();
        for j in 0..n - 1 {
            if r[(a[j], i)] < r[(a[j + 1], i)] {
                cur.insert(j);
            }
        }
        if cur.is_empty() {
            heap.push(Reverse(i));
        }
        bad.push(cur);
    }
    let mut resolved = BitSet::new(n - 1);
    let mut ans = Vec::with_capacity(k);
    while let Some(Reverse(cur)) = heap.pop() {
        ans.push(cur);
        for j in 0..n - 1 {
            if !resolved[j] && r[(a[j], cur)] > r[(a[j + 1], cur)] {
                resolved.set(j);
                for i in 0..k {
                    if bad[i].remove(&j) && bad[i].is_empty() {
                        heap.push(Reverse(i));
                    }
                }
            }
        }
    }
    if ans.len() == k {
        out.print_line(ans.inc());
    } else {
        out.print_line(-1);
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
