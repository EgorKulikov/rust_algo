//{"name":"G1. Yunli's Subarray Queries (easy version)","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/G1","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n7 5 3\n1 2 3 2 1 2 3\n1 5\n2 6\n3 7\n8 4 2\n4 3 1 1 2 4 3 2\n3 6\n2 5\n5 4 2\n4 5 1 2 3\n1 4\n2 5\n","output":"2\n3\n2\n2\n2\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1YunlisSubarrayQueriesEasyVersion"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let queries = input.read_size_pair_vec(q);

    let mut ans = Vec::with_capacity(n - k + 1);
    let mut set = BTreeSet::new();
    let mut qty = vec![0; 2 * n];
    for i in 0..k - 1 {
        let val = a[i] + n - i;
        if qty[val] != 0 {
            set.remove(&(qty[val], val));
        }
        qty[val] += 1;
        set.insert((qty[val], val));
    }
    for i in k - 1..n {
        let val = a[i] + n - i;
        if qty[val] != 0 {
            set.remove(&(qty[val], val));
        }
        qty[val] += 1;
        set.insert((qty[val], val));
        ans.push(k - set.iter().next_back().unwrap().0);
        let val = a[i - (k - 1)] + n - (i - (k - 1));
        set.remove(&(qty[val], val));
        qty[val] -= 1;
        if qty[val] != 0 {
            set.insert((qty[val], val));
        }
    }
    for (l, _) in queries {
        out.print_line(ans[l - 1]);
    }
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
