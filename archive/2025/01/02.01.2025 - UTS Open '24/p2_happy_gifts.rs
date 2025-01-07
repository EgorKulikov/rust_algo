//{"name":"P2 - Happy Gifts","group":"DMOJ - UTS Open '24","url":"https://dmoj.ca/problem/utso24p2","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n1 2 -6 -2 3\n","output":"11\n"},{"input":"3 1\n-3 -1 -4\n","output":"0\n"},{"input":"3 3\n1 1 -2\n","output":"3\n"},{"input":"6 5\n-7 4 -6 4 -9 -7\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2HappyGifts"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let h = input.read_long_vec(n);

    let mut pos = Vec::new();
    let mut neg = Vec::new();
    for i in h {
        if i > 0 {
            pos.push(i);
        } else if i < 0 {
            neg.push(-i);
        }
    }
    pos.sort_by_key(|&x| -x);
    neg.sort_by_key(|&x| -x);
    let s_pos = pos.partial_sums();
    let s_neg = neg.partial_sums();
    let mut ans = None;
    for i in 0..=neg.len().min(k / 2) {
        let rem = k - 2 * i;
        ans.maxim(s_neg[i] + s_pos[rem.min(pos.len())]);
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
