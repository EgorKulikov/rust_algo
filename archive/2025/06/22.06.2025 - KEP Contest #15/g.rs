//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_i128_vec(n);

    let pp = a.partial_sums();
    let mut qq = vec![pp[1]];
    let mut max_prefix = pp[1].max(0);
    let mut sum = pp[1];
    let mut sums = vec![pp[1]];
    for i in 2..=n {
        max_prefix.maxim(pp[i]);
        qq.push((sum + max_prefix).max(qq[i - 2]));
        sum += pp[i];
        sums.push(sum);
    }
    let mut p = vec![0];
    for i in 1..=n {
        if pp[i] > p[Back(0)] {
            p.push(pp[i]);
        } else {
            p.push(p[Back(0)]);
        }
    }

    for _ in 0..q {
        let s = input.read_i128();
        if s <= qq[0] {
            out.print_line(1);
            continue;
        }
        let p1 = qq.lower_bound(&s);
        if p1 == n {
            out.print_line(-1);
            continue;
        }
        let p2 = p.lower_bound(&(s - sums[p1 - 1]));
        out.print_line(p1 * (p1 + 1) / 2 + p2);
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
