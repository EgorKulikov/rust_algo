//{"name":"F. Algorithm Mania","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n2 3 4 5 6\n1 2 3 4 5\n","output":"2\n"},{"input":"4\n1 2 4 2\n5 6 9 7\n","output":"0\n"},{"input":"4\n9 19 6 5\n20 3 16 19\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_last_false;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_long_vec(n);
    let p = input.read_long_vec(n);

    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| m[i] + p[i]);
    let ans = search_last_false(0, n / 2, |x| {
        let mut v = Vec::with_capacity(n - x + 1);
        v.push(0);
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for i in 0..n - x {
            heap.push(p[order[i]]);
            sum += p[order[i]];
            if heap.len() > x {
                sum -= heap.pop().unwrap();
            }
            v.push(sum);
        }
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for i in (x..n).rev() {
            heap.push(Reverse(m[order[i]]));
            sum += m[order[i]];
            if heap.len() > x {
                sum -= heap.pop().unwrap().0;
            }
            if heap.len() == x && sum >= v[i] {
                return false;
            }
        }
        true
    });
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
