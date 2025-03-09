//{"name":"T533150 「o.OI R1」飞起来","group":"Luogu","url":"https://www.luogu.com.cn/problem/T533150?contestId=224200","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n3 7\n2 4\n3\n-3 1 2\n-2 -1 5\n3\n2 9 15\n17 -9 18\n","output":"T 3 7\nDraw 1 2\nC 2 15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_int_vec(n).sorted();
    let t = input.read_int_vec(n).sorted();

    let mut draw = None;
    for i in 0..n - 1 {
        let mut min_length = 1;
        let pos = t.lower_bound(&c[i]);
        if pos > 1 {
            min_length.maxim(t[pos - 1] - t[0]);
        }
        if c[i] < t[n - 1] {
            let pos = t.lower_bound(&((c[i] + t[n - 1] + 1) / 2));
            min_length.maxim(t[n - 1] - t[pos]);
            if pos > 0 {
                min_length.maxim(t[pos - 1] - c[i]);
            }
        }
        if c[n - 1] - c[i] < min_length {
            continue;
        }
        let j = c.lower_bound(&(c[i] + min_length));
        for j in j..(j + 2).min(n) {
            let pos1 = t.lower_bound(&c[i]);
            let pos2 = t.lower_bound(&c[j]);
            if pos2 - pos1 > 1 {
                continue;
            }
            let mut is_draw = false;
            if pos1 > 0 && pos1 < n {
                if t[pos1] - t[pos1 - 1] < c[j] - c[i] {
                    continue;
                }
                if t[pos1] - t[pos1 - 1] == c[j] - c[i] {
                    is_draw = true;
                }
            }
            if pos2 > 0 && pos2 < n {
                if t[pos2] - t[pos2 - 1] < c[j] - c[i] {
                    continue;
                }
                if t[pos2] - t[pos2 - 1] == c[j] - c[i] {
                    is_draw = true;
                }
            }
            if pos1 > 1 {
                if t[pos1 - 1] - t[0] > c[j] - c[i] {
                    continue;
                }
                if t[pos1 - 1] - t[0] == c[j] - c[i] {
                    is_draw = true;
                }
            }
            if pos2 < n - 1 {
                if t[n - 1] - t[pos2] > c[j] - c[i] {
                    continue;
                }
                if t[n - 1] - t[pos2] == c[j] - c[i] {
                    is_draw = true;
                }
            }
            if !is_draw {
                out.print_line(("C", c[i], c[j]));
                return;
            }
            draw = Some((c[i], c[j]));
        }
    }
    if let Some((a, b)) = draw {
        out.print_line(("Draw", a, b));
    } else {
        out.print_line(("T", c[0], c[n - 1]));
    }
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
