//{"name":"Normal is Good","group":"CodeChef - START157A","url":"https://www.codechef.com/START157A/problems/NORMAL","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n3 1 2\n4\n1 2 1 2\n5\n3 2 2 3 1\n","output":"4\n4\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NormalIsGood"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut cur_balance = 0;
    let mut uncomitted_balances = vec![0];
    let mut ones = 0;
    let mut threes = 0;
    let mut per_balance = DefaultHashMap::<_, usize>::new();
    let mut ans = 0;
    for i in a {
        match i {
            1 => {
                ones += 1;
                ans += ones;
                threes = 0;
                cur_balance += 1;
            }
            3 => {
                threes += 1;
                ans += threes;
                ones = 0;
                cur_balance -= 1;
            }
            2 => {
                for i in uncomitted_balances {
                    per_balance[i] += 1;
                }
                uncomitted_balances = Vec::new();
                ones = 0;
                threes = 0;
            }
            _ => unreachable!(),
        }
        ans += per_balance[cur_balance];
        uncomitted_balances.push(cur_balance);
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
