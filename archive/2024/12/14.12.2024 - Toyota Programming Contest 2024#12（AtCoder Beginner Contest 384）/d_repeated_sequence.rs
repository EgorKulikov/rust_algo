//{"name":"D - Repeated Sequence","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 42\n3 8 4\n","output":"Yes\n"},{"input":"3 1\n3 8 4\n","output":"No\n"},{"input":"20 83298426\n748 169 586 329 972 529 432 519 408 587 138 249 656 114 632 299 984 755 404 772\n","output":"Yes\n"},{"input":"20 85415869\n748 169 586 329 972 529 432 519 408 587 138 249 656 114 632 299 984 755 404 772\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRepeatedSequence"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut s = input.read_size();
    let a = input.read_size_vec(n);

    out.set_bool_output(BoolOutput::YesNo);
    let p = a.partial_sums();
    let ss = p[n];
    s %= ss;
    let mut aa = a.clone();
    aa.extend_from_slice(&a);
    let p = aa.partial_sums();
    let all = p.copy_iter().collect::<FxHashSet<_>>();
    for i in 0..n {
        if all.contains(&(s + p[i])) {
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
