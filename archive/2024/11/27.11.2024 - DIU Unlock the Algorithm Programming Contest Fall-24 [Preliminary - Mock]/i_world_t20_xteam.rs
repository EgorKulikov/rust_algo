//{"name":"I. World T20 X Team","group":"Toph","url":"https://toph.co/arena?contest=diu-unlock-the-algorithm-fall-24-preliminary-mock#!/p/6745ebf0cb14b2a6cffe558b","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n1 2 9 4\n2\n4\n","output":"8\n26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IWorldT20XTeam"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n).sorted();

    let mut i = 1;
    let mut j = n - 2;
    let mut sum_front = a[0];
    let mut sum_back = a[n - 1];
    let mut cur = a[n - 1] - a[0];
    let mut ans = vec![cur];
    while i <= j {
        let add_i = (a[i] * i as i64 - sum_front) + (sum_back - (n - j - 1) as i64 * a[i]);
        let add_j = (a[j] * i as i64 - sum_front) + (sum_back - a[j] * (n - j - 1) as i64);
        if add_i > add_j {
            sum_front += a[i];
            cur += add_i;
            i += 1;
        } else {
            sum_back += a[j];
            cur += add_j;
            j -= 1;
        }
        ans.push(cur);
    }
    for _ in 0..q {
        let x = input.read_size();
        out.print_line(ans[x - 2]);
    }
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
