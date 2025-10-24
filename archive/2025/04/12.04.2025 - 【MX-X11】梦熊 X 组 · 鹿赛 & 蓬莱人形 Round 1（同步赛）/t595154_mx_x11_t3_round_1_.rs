//{"name":"T595154 【MX-X11-T3】「蓬莱人形 Round 1」科学","group":"Luogu","url":"https://www.luogu.com.cn/problem/T595154?contestId=240581","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n5 2 1 3 2\n3 3\n1 5\n1 4\n","output":"3 3\n"},{"input":"5 5\n1 1 7 4 7\n5 2\n5 7\n1 6\n1 5\n2 10\n","output":"4 7\n"},{"input":"5 3\n1 2 5 4 4\n3 5\n4 4\n2 1\n","output":"3 10\n"},{"input":"5 3\n2 4 3 3 4\n1 1\n4 4\n5 2\n","output":"3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_first_true;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n).sorted().reversed();
    let mut bw = input.read_long_pair_vec(m).sorted();

    let ans = search_first_true(a[0].upper_div(2), a[0], |x| {
        let mut need = Vec::new();
        for i in 0..n {
            if a[i] > x {
                let mut cur = a[i] - x;
                if 2 * i <= n - 1 {
                    cur.minim(a[Back(i)]);
                }
                need.push(cur);
            } else {
                break;
            }
        }
        need.sort();
        let mut at = 0;
        for i in need {
            while at < m && bw[at].0 < i {
                at += 1;
            }
            if at == m {
                return false;
            }
            at += 1;
        }
        true
    });
    let mut need = Vec::new();
    for i in 0..n {
        if a[i] > ans {
            let mut cur = a[i] - ans;
            if 2 * i <= n - 1 {
                cur.minim(a[Back(i)]);
            }
            need.push(cur);
        } else {
            break;
        }
    }
    need.sort();
    bw.reverse();
    let mut at = 0;
    let mut heap = BinaryHeap::new();
    let mut cost = 0;
    for i in need.copy_rev() {
        while at < m && bw[at].0 >= i {
            heap.push(Reverse(bw[at].1));
            at += 1;
        }
        cost += heap.pop().unwrap().0;
    }
    out.print_line((ans, cost));
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
