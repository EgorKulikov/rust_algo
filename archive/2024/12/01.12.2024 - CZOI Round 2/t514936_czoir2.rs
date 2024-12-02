//{"name":"T514936 「CZOI-R2」加训","group":"Luogu","url":"https://www.luogu.com.cn/problem/T514936?contestId=203857","interactive":false,"timeLimit":2000,"tests":[{"input":"10 2\n2 2 2\n1 1\n1 2\n2 1\n2 3\n3 1\n3 2\n","output":"0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T514936CZOIR2"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let x = input.read_size();
    let y = input.read_size();

    let mut ioers = FxHashSet::default();
    for _ in 0..m {
        ioers.insert(input.read_size_vec(k));
    }
    let mut obstacles = FxHashSet::default();
    for _ in 0..x {
        obstacles.insert(input.read_size_vec(k));
    }

    let mut coaches = Vec::with_capacity(y);

    for _ in 0..y {
        let coach = input.read_size_vec(k);
        coaches.push(coach.clone());
        obstacles.insert(coach);
    }

    let mut res = Vec::with_capacity(y);
    for coach in coaches {
        let mut ans = 0;
        for i in 0..k {
            let mut cur = coach.clone();
            while cur[i] > 1 {
                cur[i] -= 1;
                if obstacles.contains(&cur) {
                    break;
                }
                if ioers.contains(&cur) {
                    ans += 1;
                    break;
                }
            }
            let mut cur = coach.clone();
            while cur[i] < n {
                cur[i] += 1;
                if obstacles.contains(&cur) {
                    break;
                }
                if ioers.contains(&cur) {
                    ans += 1;
                    break;
                }
            }
        }
        res.push(ans);
    }

    out.print_line(res);
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
