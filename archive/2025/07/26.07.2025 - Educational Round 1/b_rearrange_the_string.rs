//{"name":"B. Rearrange the String","group":"SeriousOJ - Educational Round 1","url":"https://judge.eluminatis-of-lu.com/contest/686fe616d425270007014c27/1209","interactive":false,"timeLimit":1000,"tests":[{"input":"10\ncc\nbcbbbcac\nacbcb\nb\nbcbbbccacb\naabca\ncbcccacbc\ncccabcaaa\nbca\nbbabc\n","output":"-1\nabcbcbcb\nabcbc\nb\nabcbcbcbcb\nabaca\n-1\nabcacacac\nabc\nbabcb\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::qty::StrQty;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut qty = s.qty_lower();
    fn check(q: &Vec<usize>) -> bool {
        q.copy_max() * 2 <= q.copy_sum() + 1
    }
    if !check(&qty) {
        out.print_line("-1");
        return;
    }
    let mut ans = Str::new();
    let mut forbidden = 26;
    for _ in 0..s.len() {
        for i in 0..26 {
            if qty[i] > 0 && i != forbidden {
                qty[i] -= 1;
                if check(&qty) {
                    ans.push((i as u8) + b'a');
                    forbidden = i;
                    break;
                }
                qty[i] += 1;
            }
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
