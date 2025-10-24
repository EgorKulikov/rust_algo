//{"name":"Clutch or kick","group":"Eolymp - Basecamp - Weekend Practice #9","url":"https://basecamp.eolymp.com/en/compete/3hdr42rsv164r0602arq87cfu8/problem/5","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n8 5\n1 2 -4 5 -8 4 5 3\n6 3\n1 1 -100 1 100 -100\n1 10\n20\n","output":"5 5 5 5 5 5 5 5\n3 3 3 3 3 1\nx\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use std::collections::{BTreeMap, BTreeSet};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long();
    let a = input.read_long_vec(n);

    let ps = a.partial_sums();
    let mut right = BTreeMap::new();
    let mut left = BTreeMap::new();
    let mut variants = BTreeSet::new();
    let mut sum = 0;
    for i in 0..n {
        right.insert((sum, i), None);
        sum += a[i];
    }
    right.insert((sum, n), None);
    sum = 0;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(pos) = right.remove(&(sum, i)).unwrap() {
            variants.remove(&(sum - ps[pos], pos, i));
            left.insert((ps[pos], pos), None);
            if let Some(last) = right.prev(&(ps[pos] + s, n + 1)) {
                if last.1.is_none() {
                    let key = *last.0;
                    right.insert(key, Some(pos));
                    left.insert((ps[pos], pos), Some(key.1));
                    variants.insert((key.0 - ps[pos], pos, key.1));
                }
            }
        }
        if let Some((&key, &link)) = right.prev(&(sum + s, n + 1)) {
            if link.is_none() || ps[link.unwrap()] > sum {
                if let Some(x) = link {
                    variants.remove(&(key.0 - ps[x], x, key.1));
                    left.insert((ps[x], x), None);
                }
                right.insert(key, Some(i));
                left.insert((sum, i), Some(key.1));
                variants.insert((key.0 - sum, i, key.1));
            } else {
                left.insert((sum, i), None);
            }
        } else {
            left.insert((sum, i), None);
        }
        if let Some(&(val, _, _)) = variants.last() {
            ans.push(Some(val));
        } else {
            ans.push(None);
        }
        sum += a[i];
    }
    let mut first = true;
    for x in ans {
        if !first {
            out.print(" ");
        }
        first = false;
        if let Some(x) = x {
            out.print(x);
        } else {
            out.print("x");
        }
    }
    out.print_line(());
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
