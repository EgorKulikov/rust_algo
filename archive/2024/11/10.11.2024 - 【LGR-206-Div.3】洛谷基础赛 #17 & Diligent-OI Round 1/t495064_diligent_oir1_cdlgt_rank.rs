//{"name":"T495064 「Diligent-OI R1 C」DlgtRank","group":"Luogu","url":"https://www.luogu.com.cn/problem/T495064?contestId=127930","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1\n1 3 4\n","output":"1 1 0\n"},{"input":"6 2\n1 2 5 6 7 8\n","output":"4 3 3 2 1 0\n"},{"input":"8 10\n1 14 5 14 19 19 8 10\n","output":"5 1 4 1 0 0 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T495064DiligentOIR1CDlgtRank"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let order = (0..n)
        .collect_vec()
        .do_with(|order| order.sort_by_key(|&i| Reverse(a[i])));
    let mut last = usize::MAX;
    let mut last_ans = 0usize;
    let mut ans = vec![0; n];
    for i in order {
        if a[i] == last {
            ans[i] = last_ans;
            continue;
        }
        let fit = (last - a[i] - 1) / k;
        ans[i] = (last_ans + 1).saturating_sub(fit);
        last = a[i];
        last_ans = ans[i];
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
