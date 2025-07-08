//{"name":"F1. Appending Permutations (Easy Version)","group":"Codeforces - EPIC Institute of Technology Round Summer 2025 (Codeforces Round 1036, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2124/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 0\n3 3\n1 1\n2 1\n3 1\n3 2\n1 1\n2 1\n6 2\n2 3\n4 2\n2 3\n1 2\n2 2\n1 1\n4 3\n2 2\n3 2\n4 2\n3 2\n2 3\n3 3\n","output":"7\n0\n1\n65\n0\n4\n5\n"},{"input":"1\n100 1\n69 69\n","output":"381055417\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut forb = vec![BitSet::new(n); n];
    for (i, x) in input.iter::<(usize, usize)>().take(m) {
        forb[i - 1].set(x - 1);
    }

    type Mod = ModIntF;
    let mut can = Memoization2d::new(n + 1, n + 1, |mem, pos: usize, val: usize| -> usize {
        if pos == n || val == n || forb[pos][val] {
            0
        } else {
            1 + mem.call(pos + 1, val + 1)
        }
    });
    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, pos: usize, not: usize| {
        if pos == n {
            Mod::one()
        } else {
            let mut res = Mod::zero();
            for r in 0..n - pos {
                if not == r {
                    continue;
                }
                for s in r + 1..=n - pos {
                    if can.call(pos, r) >= s - r && can.call(pos + s - r, 0) >= r {
                        res += mem.call(pos + s, if r == 0 { s } else { n });
                    }
                }
            }
            res
        }
    });
    out.print_line(mem.call(0, n));
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
