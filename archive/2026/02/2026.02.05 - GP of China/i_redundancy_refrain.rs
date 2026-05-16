//{"name":"I. Redundancy Refrain","group":"Universal Cup - GP of China","url":"https://contest.ucup.ac/contest/3295/problem/16336","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n8 3\n0 1 0 2 1 3 0 2\n3 3\n1 0 2\n9 7\n0 1 1 0 5 0 4 0 7\n6 6\n0 0 0 0 0 0\n","output":"1 1 1 2 1 3 2 2\n1 3 2\n2 1 1 5 5 4 4 3 7\n1 2 3 4 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut min = vec![None; k + 1];
    let mut max = vec![None; k + 1];
    for i in 0..n {
        if a[i] != 0 {
            min[a[i]].minim(i);
            max[a[i]].maxim(i);
        }
    }
    let mut free = Vec::new();
    let mut ends = vec![Vec::new(); n];
    let mut min_ans = 0;
    for i in 1..=k {
        if let (Some(l), Some(r)) = (min[i], max[i]) {
            ends[r].push((l, i));
            min_ans.maxim(r - l);
        } else {
            free.push(i);
        }
    }

    let solve = |delta: usize| -> Option<Vec<usize>> {
        let mut free = free.clone();
        let mut heap = BinaryHeap::new();
        let mut good_until = 0;
        let mut cur = k;

        for i in 0..delta {
            for l in ends[i].copy_iter() {
                heap.push(Reverse(l));
            }
        }
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            if i + delta < n {
                for l in ends[i + delta].copy_iter() {
                    heap.push(Reverse(l));
                }
            }
            if a[i] != 0 {
                res.push(a[i]);
            } else if i < good_until {
                res.push(cur);
            } else {
                let mut found = false;
                while let Some(Reverse((from, col))) = heap.pop() {
                    if i <= from + delta {
                        cur = col;
                        good_until = i.min(from) + delta + 1;
                        found = true;
                        break;
                    }
                }
                if found {
                    res.push(cur);
                } else if let Some(col) = free.pop() {
                    cur = col;
                    good_until = i + delta + 1;
                    res.push(cur);
                } else {
                    return None;
                }
            }
        }
        Some(res)
    };

    let mut l = min_ans;
    let mut r = n - 1;
    while l < r {
        let mid = (l + r) / 2;
        if solve(mid).is_some() {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    eprintln!("Chosen delta: {}", l);
    out.print_line(solve(l));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
