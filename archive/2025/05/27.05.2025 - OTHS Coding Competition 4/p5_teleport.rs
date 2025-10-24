//{"name":"P5 - Teleport","group":"DMOJ - OTHS Coding Competition 4","url":"https://dmoj.ca/problem/othscc4p5","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n...\n..x\n.x.\n","output":"6\n"},{"input":"5\nx....\n.x...\n...x.\n.....\n.x..x\n","output":"25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_char_table(n, n);

    let mut first = vec![None; n];
    let mut last = vec![None; n];
    for i in 0..n {
        for j in 0..n {
            if a[(i, j)] == b'x' {
                first[i].minim(j);
                last[i].maxim(j);
            }
        }
    }
    for i in 1..n {
        if let Some(x) = first[i - 1] {
            first[i].minim(x);
        }
    }
    for i in (0..n - 1).rev() {
        if let Some(x) = last[i + 1] {
            last[i].maxim(x);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if let Some(f) = first[i] {
            if let Some(l) = last[i] {
                if l >= f {
                    ans += l - f + 1;
                    for j in f..=l {
                        a[(i, j)] = b'x';
                    }
                }
            }
        }
    }
    if ans == 0 {
        out.print_line(2 * n - 1);
        return;
    }
    let mut prefix = 0;
    let mut suffix = 0;
    for i in 0..n {
        for j in 0..n {
            if a[(i, j)] == b'x' {
                if i > 0 && a[(i - 1, j)] != b'x' || j > 0 && a[(i, j - 1)] != b'x' {
                    prefix.maxim(i + j);
                }
                if i + 1 < n && a[(i + 1, j)] != b'x' || j + 1 < n && a[(i, j + 1)] != b'x' {
                    suffix.maxim(n - i + n - j - 2);
                }
            }
        }
    }
    out.print_line(ans + prefix + suffix);
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
