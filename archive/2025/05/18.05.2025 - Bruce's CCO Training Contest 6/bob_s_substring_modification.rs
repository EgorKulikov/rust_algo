//{"name":"Bob's Substring Modification","group":"DMOJ","url":"https://dmoj.ca/problem/oly21practice36","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\naabaa\nababa\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::{BTreeSet, BinaryHeap};

use algo_lib::misc::test_type::TestType;
use algo_lib::string::concat::StrConcat;
use algo_lib::string::str::StrReader;
use algo_lib::string::suffix_array::SuffixArray;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_str();
    let b = input.read_str();

    let s = a.str_concat(&b);
    let sa = SuffixArray::new(&s);

    let mut ans = 0;
    let mut last_a = 2 * n + 1;
    let mut last_b = 2 * n + 1;
    let mut heap = BinaryHeap::new();
    let mut a = BTreeSet::new();
    let mut b = BTreeSet::new();
    for i in 1..=2 * n {
        if sa[i] % n + k > n {
            continue;
        }
        if sa[i] < n {
            if last_b <= 2 * n {
                heap.push((sa.lcp(i, last_b), i, last_b));
            }
            last_a = i;
            a.insert(i);
        } else {
            if last_a <= 2 * n {
                heap.push((sa.lcp(i, last_a), last_a, i));
            }
            last_b = i;
            b.insert(i);
        }
    }
    let mut used = BitSet::new(2 * n + 1);
    while let Some((len, i, j)) = heap.pop() {
        if used[i] || used[j] {
            continue;
        }
        used.set(i);
        used.set(j);
        ans += k - k.min(len);
        a.remove(&i);
        b.remove(&j);
        if let Some(&x) = a.prev(&i) {
            if let Some(&y) = b.next(&x) {
                heap.push((sa.lcp(x, y), x, y));
            }
        }
        if let Some(&x) = a.next(&i) {
            if let Some(&y) = b.prev(&x) {
                heap.push((sa.lcp(x, y), x, y));
            }
        }
        if let Some(&y) = b.prev(&j) {
            if let Some(&x) = a.next(&y) {
                heap.push((sa.lcp(x, y), x, y));
            }
        }
        if let Some(&y) = b.next(&j) {
            if let Some(&x) = a.prev(&y) {
                heap.push((sa.lcp(x, y), x, y));
            }
        }
    }
    out.print_line(ans);
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
