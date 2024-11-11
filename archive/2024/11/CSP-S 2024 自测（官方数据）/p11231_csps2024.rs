//{"name":"P11231 [CSP-S 2024] 决斗（官方数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11231?contestId=209925","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2 3 1 2\n","output":"2\n"},{"input":"10\n136 136 136 2417 136 136 2417 136 136 136\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11231CSPS2024"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_int_vec(n).sorted();

    let mut current = 0;
    let mut qty = 0;
    let mut total = 0;
    let mut ans = 0;
    for i in r {
        if i != current {
            total += qty;
            current = i;
            qty = 0;
        }
        qty += 1;
        if total > 0 {
            total -= 1;
            ans += 1;
        }
    }
    out.print_line(n - ans);
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
