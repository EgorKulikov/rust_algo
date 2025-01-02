//{"name":"Low cost water management ","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1156","interactive":false,"timeLimit":1000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LowCostWaterManagement"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_long();
    let y = input.read_long();
    let maze = input.read_char_table(n, m);

    let mut cur = vec![None; m];
    cur[0] = Some(0);
    for i in 1..m {
        if maze[(0, i)] == b'.' {
            cur[i] = Some((i - 1) as i64 * x + y);
        } else {
            break;
        }
    }
    for i in 1..n - 1 {
        let mut next = vec![None; m];
        let mut from_left = None;
        for j in 0..m {
            if maze[(i, j)] == b'#' {
                from_left = None;
                continue;
            }
            if let Some(val) = from_left {
                next[j].minim(val + y);
            }
            from_left = from_left.map(|val| val + x);
            if let Some(val) = cur[j] {
                next[j].minim(val + x);
                from_left.minim(val + y);
            }
        }
        let mut from_right = None;
        for j in (0..m).rev() {
            if maze[(i, j)] == b'#' {
                from_right = None;
                continue;
            }
            if let Some(val) = from_right {
                next[j].minim(val + y);
            }
            from_right = from_right.map(|val| val + x);
            if let Some(val) = cur[j] {
                from_right.minim(val + y);
            }
        }
        cur = next;
    }
    let mut ans = None;
    if let Some(val) = cur[m - 1] {
        ans.minim(val);
    }
    for j in (0..m - 1).rev() {
        if maze[(n - 1, j)] == b'#' {
            break;
        }
        if let Some(val) = cur[j] {
            ans.minim(val + (m - 2 - j) as i64 * x + y);
        }
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
