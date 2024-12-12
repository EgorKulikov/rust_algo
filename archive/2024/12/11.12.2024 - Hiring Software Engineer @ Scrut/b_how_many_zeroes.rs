//{"name":"B - How Many Zeroes?","group":"LightOJ","url":"https://lightoj.com/contest/scrutcoding/arena/problem/6491","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 11\n100 200\n0 500\n1234567890 2345678901\n0 4294967295\n","output":"Case 1: 1\nCase 2: 22\nCase 3: 92\nCase 4: 987654304\nCase 5: 3825876150\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHowManyZeroes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_iterator::iterate;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();

    let mut ans = 0;
    let tens = powers(10usize, 10);
    for (mut prefix, len, _) in iterate(m, n) {
        let mut prefix_zeros = 0;
        while prefix > 0 {
            if prefix % 10 == 0 {
                prefix_zeros += 1;
            }
            prefix /= 10;
        }
        ans += prefix_zeros * tens[len];
        if len > 0 {
            ans += len * tens[len - 1];
        }
    }
    if m == 0 {
        ans += 1;
    }
    output!(out, "Case {}: {}", test_case, ans);
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
