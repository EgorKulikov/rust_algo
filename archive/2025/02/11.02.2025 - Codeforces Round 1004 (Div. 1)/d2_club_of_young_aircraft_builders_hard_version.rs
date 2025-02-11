//{"name":"D2. Club of Young Aircraft Builders (hard version)","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/D2","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n3 2 4\n0 0 0 0\n5 5 7\n0 0 0 0 0 0 0\n6 1 3\n2 0 0\n2 3 5\n0 0 1 0 2\n3 3 4\n3 3 3 0\n2 1 2\n0 1\n2 1 2\n0 2\n5 3 12\n0 0 1 0 2 4 0 0 0 5 0 5\n","output":"6\n190\n3\n2\n0\n0\n1\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(m);

    let gaps = a.copy_count(0);
    let last = a.copy_count(n);
    if last > c {
        out.print_line(0);
        return;
    }
    if gaps + last < c {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let cc = Combinations::<Mod>::new(m + 1);
    let more_or_gaps = Vec::with_gen(n + 1, |i| {
        Vec::with_gen_prefix(m + 1, |j, v| {
            if j == 0 {
                0
            } else {
                v[j - 1] + if a[j - 1] == 0 || a[j - 1] >= i { 1 } else { 0 }
            }
        })
    });
    let last_seen = Vec::with_gen(n + 1, |i| {
        for j in (0..m).rev() {
            if a[j] == i {
                return j;
            }
        }
        0
    });
    let zeroes = Vec::with_gen_prefix(m + 1, |j, v| {
        if j == 0 {
            0
        } else {
            v[j - 1] + if a[j - 1] == 0 { 1 } else { 0 }
        }
    });
    let mut mem = Memoization2d::new(n + 1, gaps + last - c + 1, |mem, fl, filled| {
        if fl == n {
            if filled == gaps + last - c {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let pos = more_or_gaps[fl].lower_bound(&(c + filled));
            if last_seen[fl] >= pos {
                return Mod::zero();
            }
            let num_gaps = zeroes[pos];
            assert!(num_gaps >= filled);
            let mut res = Mod::zero();
            for i in 0..=num_gaps - filled {
                if filled + i > gaps + last - c {
                    break;
                }
                res += mem.call(fl + 1, filled + i) * cc.c(num_gaps - filled, i);
            }
            res
        }
    });
    out.print_line(mem.call(1, 0));
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

//START MAIN
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
//END MAIN
