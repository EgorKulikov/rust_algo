//{"name":"D. Перестановочная черная дыра","group":"Codeforces - Codeforces Round 1040 (Div. 1)","url":"https://codeforces.com/contest/2129/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"9\n3\n-1 -1 1\n3\n-1 -1 -1\n4\n-1 2 -1 0\n4\n-1 0 1 -1\n5\n-1 3 -1 0 -1\n5\n4 4 4 4 4\n5\n1 0 1 2 0\n6\n-1 1 -1 -1 3 0\n13\n-1 -1 -1 -1 -1 -1 2 -1 -1 -1 -1 -1 -1\n","output":"2\n6\n4\n3\n8\n0\n4\n10\n867303072\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_int_vec(n);

    const BUBEN: usize = 15;

    let mut sum = 0;
    for i in 0..n {
        if s[i] != -1 {
            sum += s[i];
        }
        if s[i] >= BUBEN as i32 - 1 {
            out.print_line(0);
            return;
        }
    }
    if sum >= n as i32 {
        out.print_line(0);
        return;
    }
    type Mod = ModIntF;
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut mem = Memoization4d::new(n + 1, n + 1, BUBEN, BUBEN, |mem, f, t, down, up| {
        if f == t {
            if (down == 0 || down == BUBEN - 1) && (up == 0 || up == BUBEN - 1) {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let mut res = Mod::zero();
            for i in f..t {
                let mut n_up = up;
                let mut n_down = down;
                let mut delta_up = 0;
                let mut delta_down = 0;
                if f == 0 {
                    if t != n {
                        delta_up = 1;
                    }
                } else {
                    if t == n {
                        delta_down = 1;
                    } else if (i - f) <= (t - 1 - i) {
                        delta_down = 1;
                    } else {
                        delta_up = 1;
                    }
                }
                if n_up < delta_up || n_down < delta_down {
                    continue;
                }
                if n_up != BUBEN - 1 {
                    n_up -= delta_up;
                }
                if n_down != BUBEN - 1 {
                    n_down -= delta_down;
                }
                if s[i] == -1 {
                    res += mem.call(f, i, n_down, BUBEN - 1)
                        * mem.call(i + 1, t, BUBEN - 1, n_up)
                        * c.c(t - f - 1, i - f);
                } else {
                    let sum = s[i] as usize;
                    for left in 0..=sum {
                        res += mem.call(f, i, n_down, left)
                            * mem.call(i + 1, t, sum - left, n_up)
                            * c.c(t - f - 1, i - f);
                    }
                }
            }
            res
        }
    });
    out.print_line(mem.call(0, n, 0, 0));
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
