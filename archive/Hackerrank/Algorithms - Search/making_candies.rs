//{"name":"Making Candies","group":"HackerRank - Algorithms - Search","url":"https://www.hackerrank.com/challenges/making-candies/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"3 1 2 12\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MakingCandies"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_i128();
    let w = input.read_i128();
    let p = input.read_i128();
    let n = input.read_i128();

    let mut ans = n.upper_div(m * w);
    let mut m = m;
    let mut w = w;
    let mut rem = 0;
    let mut done = 0;
    loop {
        let need_steps = if rem + m * w >= p {
            1
        } else {
            (p - rem).upper_div(m * w).max(1)
        };
        done += need_steps;
        rem += need_steps * m * w;
        if rem >= n {
            break;
        }
        while rem >= p && m * w < n {
            rem -= p;
            if m < w {
                m += 1;
            } else {
                w += 1;
            }
            ans.minim((n - rem).upper_div(m * w) + done);
        }
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
