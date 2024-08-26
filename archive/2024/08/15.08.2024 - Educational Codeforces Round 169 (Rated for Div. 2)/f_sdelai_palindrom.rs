//{"name":"F. Сделай палиндром","group":"Codeforces - Educational Codeforces Round 169 (Rated for Div. 2)","url":"https://codeforces.com/contest/2004/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n3\n2 1 3\n4\n1 1 1 1\n5\n4 2 3 1 5\n4\n1 2 1 2\n","output":"3\n0\n14\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSdelaiPalindrom"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let s = a.partial_sums();
    let mut ans = 0;
    let mut qty = DefaultHashMap::<_, usize>::new();
    for i in 0..n {
        for j in i + 1..=n {
            ans += j - i - 1;
            qty[s[j] - s[i]] += 1;
        }
    }
    for v in qty.into_values() {
        ans -= v * (v - 1) / 2;
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
