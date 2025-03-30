//{"name":"P6 - Robert Loves ABGs","group":"DMOJ - Arcadia Computing Contest 2","url":"https://dmoj.ca/problem/ahscc2p6","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n101\n111\n100\n000\n001\n","output":"111\n"},{"input":"3 10\n1111110000\n1111110000\n1111110000\n","output":"1111111111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let abg = input.read_str_vec(n);

    let mut can = BitSet::new(m + 1);
    can.set(0);
    let mut next = BitSet::new(m + 1);
    for s in abg {
        let q = s.copy_count(b'1');
        next.fill(false);
        for i in 0..=m {
            if can[i] {
                next.set(i);
                for same in 0..=q.min(i) {
                    let ones = i + q - 2 * same;
                    if ones + same <= m {
                        next.set(ones);
                    }
                }
            }
        }
        swap(&mut can, &mut next);
    }
    for i in (0..=m).rev() {
        if can[i] {
            let mut ans = Str::from(vec![b'0'; m]);
            ans[..i].fill(b'1');
            out.print_line(ans);
            return;
        }
    }
    unreachable!();
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
