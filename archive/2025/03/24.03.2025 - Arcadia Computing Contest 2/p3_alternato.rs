//{"name":"P3 - Alternato","group":"DMOJ - Arcadia Computing Contest 2","url":"https://dmoj.ca/problem/ahscc2p3","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 1 3 1 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut bad_pos = Vec::new();
    for i in 0..n {
        if i == 0 && a[i] == a[i + 1] {
            bad_pos.push(i);
        } else if i == n - 1 && a[i] == a[i - 1] {
            bad_pos.push(i);
        } else if i > 0
            && i < n - 1
            && !(a[i] > a[i - 1] && a[i] > a[i + 1] || a[i] < a[i - 1] && a[i] < a[i + 1])
        {
            bad_pos.push(i);
        }
    }
    if bad_pos.len() > 3 {
        out.print_line(-1);
        return;
    }
    if bad_pos.is_empty() {
        out.print_line(0);
        return;
    }

    for i in bad_pos {
        for j in [-1, 1] {
            let mut b = a.clone();
            b[i] += j;
            let mut good = true;
            for i in 0..n {
                if (i == 0 || b[i] > b[i - 1]) && (i == n - 1 || b[i] > b[i + 1]) {
                    continue;
                }
                if (i == 0 || b[i] < b[i - 1]) && (i == n - 1 || b[i] < b[i + 1]) {
                    continue;
                }
                good = false;
                break;
            }
            if good {
                out.print_line(1);
                return;
            }
        }
    }
    out.print_line(-1);
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
