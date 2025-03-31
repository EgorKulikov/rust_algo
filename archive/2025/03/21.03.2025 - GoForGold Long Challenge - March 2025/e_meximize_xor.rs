//{"name":"E. Meximize XOR","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n4\n0 1 2 3\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n).sorted();

    let mut expect = 0;
    let mut b = Vec::new();
    for i in a {
        if i == expect {
            expect += 1;
        } else {
            b.push(i);
        }
    }

    fn recalc(b: &mut Vec<u32>) -> u32 {
        let mut at = 0;
        for i in (0..31).rev() {
            let mut j = at;
            while j < b.len() && !b[j].is_set(i) {
                j += 1;
            }
            if j == b.len() {
                continue;
            }
            b.swap(at, j);
            for k in 0..at {
                if b[k].is_set(i) {
                    b[k] ^= b[at];
                }
            }
            for k in j + 1..b.len() {
                if b[k].is_set(i) {
                    b[k] ^= b[at];
                }
            }
            at += 1;
        }
        b.truncate(at);
        b.copy_fold(0, |acc, x| acc ^ x)
    }
    let mut ans = expect as i64 + recalc(&mut b) as i64;
    for i in (0..expect).rev() {
        b.push(i);
        ans.maxim(recalc(&mut b) as i64 + i as i64);
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
