//{"name":"P4 - Political Espionage","group":"DMOJ - UTS Open '24","url":"https://dmoj.ca/problem/utso24p4","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n","output":"3 1 4 1 5 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4PoliticalEspionage"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = vec![0; n];
    let mid = n / 2;
    ans[mid] = 10_000_000_000;
    let mut rec = RecursiveFunction3::new(|rec, f: usize, t: usize, q: usize| -> (usize, usize) {
        if f == t {
            return (q, 0);
        }
        assert!(q >= t - f);
        if f + 1 == t {
            ans[f] = 1;
            return (q - 1, 1);
        }
        let mid = (f + t) / 2;
        let q_mid = (q / 2 + 1).min(q - (t - f - 1));
        ans[mid] = q_mid;
        let mut rem = q - q_mid;
        let (add, right_sum) = rec.call(mid + 1, t, rem / 2);
        rem -= rem / 2;
        rem += add;
        let (mut rem, left_sum) = rec.call(f, mid, rem);
        if ans[mid] > left_sum + right_sum + 1 {
            rem += ans[mid] - left_sum - right_sum - 1;
            ans[mid] = left_sum + right_sum + 1;
        }
        (rem, ans[mid] + left_sum + right_sum)
    });
    let rem = rec.call(mid + 1, n, 10_000_000_000 / 2 - 1).0;
    rec.call(0, mid, 10_000_000_000 / 2 + rem);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
