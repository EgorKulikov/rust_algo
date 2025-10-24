//{"name":"E1. Looking at Towers (easy version)","group":"Codeforces - Educational Codeforces Round 182 (Rated for Div. 2)","url":"https://codeforces.com/contest/2144/problem/E1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n5\n4 2 4 8 3\n5\n1 2 3 2 1\n6\n1 2 3 3 2 1\n9\n3 5 5 7 4 6 7 2 4\n1\n10\n","output":"5\n1\n3\n51\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut l = Vec::new();
    for i in a.copy_iter() {
        if l.is_empty() || i > l[Back(0)] {
            l.push(i);
        }
    }
    let mut r = Vec::new();
    for i in a.copy_rev() {
        if r.is_empty() || i > r[Back(0)] {
            r.push(i);
        }
    }
    r.reverse();
    let base = l.len() - 1;
    l.extend(r);

    type Mod = ModIntF;
    let mut cur = vec![Mod::zero(); l.len() + 1];
    let mut next = vec![Mod::zero(); l.len() + 1];
    cur[0] = Mod::one();
    for i in a {
        next.fill(Mod::zero());
        for j in 0..=l.len() {
            next[j] += cur[j];
            if j > 0 && j <= base + 1 && i <= l[j - 1] {
                next[j] += cur[j];
            }
            if j > base + 1 && j < l.len() && i <= l[j] {
                next[j] += cur[j];
            }
            if j < l.len() && i == l[j] {
                next[j + 1] += cur[j];
            }
            if j == base && i == l[j] {
                next[j + 2] += cur[j];
            }
        }
        std::mem::swap(&mut cur, &mut next);
    }
    out.print_line(cur[l.len()]);
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
