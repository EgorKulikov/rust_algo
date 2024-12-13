//{"name":"T550971 202412F 顽强拼搏奖的四种发法","group":"Luogu","url":"https://www.luogu.com.cn/problem/T550971?contestId=219467","interactive":false,"timeLimit":1000,"tests":[{"input":"8 4 2 2\n1 1 1\n1 2 1\n2 2 1\n3 1 1\n4 1 1\n4 2 1\n2 1 1\n1 2 1\n","output":"1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T550971202412F"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_size();
    let p = input.read_size();
    let k = input.read_size();
    let submissions = input.read_vec::<(usize, usize, usize)>(n);

    let mut last_ac = None;
    let mut last_effective_ac = None;
    let mut possible_last_non_medal_ac = Vec::new();
    let mut last_first_ac = None;
    let mut solved = vec![0; t];
    let mut is_solved = Arr2d::new(t, p, false);
    for (tid, pid, state) in submissions {
        if state == 1 {
            last_ac = Some(tid);
            if !is_solved[tid - 1][pid - 1] {
                is_solved[tid - 1][pid - 1] = true;
                solved[tid - 1] += 1;
                last_effective_ac = Some(tid);
                if solved[tid - 1] == 1 {
                    last_first_ac = Some(tid);
                }
                possible_last_non_medal_ac.push(tid);
            }
        }
    }
    let mut last_non_medal_ac = None;
    for tid in possible_last_non_medal_ac.iter_rev() {
        if solved[tid - 1] < k {
            last_non_medal_ac = Some(tid);
            break;
        }
    }
    out.print_line((last_ac, last_effective_ac, last_non_medal_ac, last_first_ac));
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
