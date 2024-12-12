//{"name":"A - Tug of War","group":"LightOJ","url":"https://lightoj.com/contest/scrutcoding/arena/problem/6490","interactive":false,"timeLimit":6000,"tests":[{"input":"2\n\n3\n100\n90\n200\n\n4\n10\n15\n17\n20\n","output":"Case 1: 190 200\nCase 2: 30 32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATugOfWar"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size_vec(n);

    let sum = w.iter().sum::<usize>();
    let mut dp = vec![BitSet::new(sum / 2 + 1); (n + 1) / 2 + 1];
    dp[0].set(0);
    for i in 0..n {
        if w[i] > sum / 2 {
            continue;
        }
        for j in (0..((n + 1) / 2).min(i + 1)).rev() {
            let mut cp = dp[j].clone();
            cp <<= w[i];
            dp[j + 1] |= &cp;
        }
    }
    for i in (0..=sum / 2).rev() {
        if dp[n / 2][i] || dp[(n + 1) / 2][i] {
            output!(out, "Case {}: {} {}", test_case, i, sum - i);
            return;
        }
    }
    loop {}
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
