//{"name":"P11233 [CSP-S 2024] 染色（官方数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11233?contestId=209925","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 2 1\n4\n1 2 3 4\n8\n3 5 2 5 1 2 1 4\n","output":"1\n0\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11233CSPS2024"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashMap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut add = 0;
    for (&x, &y) in a.consecutive_iter() {
        if x == y {
            add += x;
        }
    }
    a.dedup();
    let mut ans = vec![0; a.len()];
    let mut last = HashMap::new();
    last.insert(a[0], 0);
    for i in 1..a.len() {
        ans[i] = ans[i - 1];
        if let Some(&j) = last.get(&a[i]) {
            let cand = a[i] + ans[j + 1];
            ans[i].maxim(cand);
        }
        last.insert(a[i], i);
    }
    out.print_line(ans[Back(0)] + add);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
