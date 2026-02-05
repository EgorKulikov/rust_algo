//{"name":"D - Pawn Line","group":"AtCoder - Denso Create Programming Contest 2026（AtCoder Beginner Contest 443）","url":"https://atcoder.jp/contests/abc443/tasks/abc443_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n5 2 1 3 4\n2\n1 1\n3\n1 3 1\n9\n9 9 8 2 4 4 3 5 3\n20\n7 4 6 2 15 5 17 15 1 8 18 1 5 1 12 11 2 7 8 14\n","output":"4\n0\n1\n16\n105\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_size_vec(n);

    let mut heap = IndexedHeap::new(n);
    for i in 0..n {
        heap.add_or_adjust(i, r[i]);
    }
    let mut done = BitSet::new(n);
    let mut ans = 0;
    while let Some((pos, val)) = heap.pop() {
        done.set(pos);
        ans += r[pos] - val;
        if pos > 0 && !done[pos - 1] {
            heap.add_or_relax(pos - 1, val + 1);
        }
        if pos + 1 < n && !done[pos + 1] {
            heap.add_or_relax(pos + 1, val + 1);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
