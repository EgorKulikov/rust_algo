//{"name":"T724554 Adverse Present","group":"Luogu","url":"https://www.luogu.com.cn/problem/T724554?contestId=304744","interactive":false,"timeLimit":1000,"tests":[{"input":"4 1\n4 3 2 1\n","output":"2\n2 1 3\n2 1 3\n"},{"input":"5 1\n5 2 3 4 1\n","output":"2\n3 2 3 4\n1 1\n"},{"input":"5 0\n5 2 3 4 1\n","output":"2\n-1\n"},{"input":"8 0\n1 8 5 4 3 2 6 7\n","output":"3\n-1\n"},{"input":"8 0\n1 3 5 7 2 4 6 8\n","output":"2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let tp = input.read_int();
    let mut p = input.read_size_vec(n).dec();

    let q = p.inv();
    let mut num_layers = 1i32;
    for i in 1..n {
        if q[i] < q[i - 1] {
            num_layers += 1;
        }
    }
    let steps = if ((num_layers - 1) & num_layers) == 0 {
        num_layers.highest_bit()
    } else {
        num_layers.highest_bit() + 1
    };
    out.print_line(steps);
    if tp == 0 || steps * n >= 3_000_000 {
        out.print_line(-1);
        return;
    }
    for _ in 0..steps {
        let q = p.inv();
        let mut even = Vec::new();
        let mut odd = Vec::new();
        even.push(q[0]);
        let mut layer = 0;
        for i in 1..n {
            if q[i] < q[i - 1] {
                layer += 1;
            }
            if layer % 2 == 0 {
                even.push(q[i]);
            } else {
                odd.push(q[i]);
            }
        }
        even.sort();
        odd.sort();
        p = even
            .into_iter()
            .chain(odd.copy_iter())
            .map(|i| p[i])
            .collect();
        out.print_line((odd.len(), odd.inc()));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
