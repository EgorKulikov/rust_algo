//{"name":"B. Building a Reactor","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17261","interactive":false,"timeLimit":1000,"tests":[{"input":"6 9\n.........\n.........\n....#....\n....#....\n.........\n.........\n","output":"100100010\n100001000\n0010#0001\n1000#0100\n000110001\n010000100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let g = input.read_char_table(n, m);

    pub struct Memoization3d<F>
    where
        F: FnMut(&mut dyn Callable3<usize, usize, usize, i32>, usize, usize, usize) -> i32,
    {
        f: std::cell::UnsafeCell<F>,
        res: Arr3d<i32>,
    }

    impl<F> Memoization3d<F>
    where
        F: FnMut(&mut dyn Callable3<usize, usize, usize, i32>, usize, usize, usize) -> i32,
    {
        pub fn new(d1: usize, d2: usize, d3: usize, f: F) -> Self {
            Self {
                f: std::cell::UnsafeCell::new(f),
                res: Arr3d::new(d1, d2, d3, i32::MIN),
            }
        }
    }

    impl<F> Callable3<usize, usize, usize, i32> for Memoization3d<F>
    where
        F: FnMut(&mut dyn Callable3<usize, usize, usize, i32>, usize, usize, usize) -> i32,
    {
        fn call(&mut self, a1: usize, a2: usize, a3: usize) -> i32 {
            if self.res[(a1, a2, a3)] != i32::MIN {
                self.res[(a1, a2, a3)]
            } else {
                let res = unsafe { (*self.f.get())(self, a1, a2, a3) };
                self.res[(a1, a2, a3)] = res;
                res
            }
        }
    }

    let prev = 3usize.power(n - 1);
    let encode = |mask: i32, ans: i16| -> i32 {
        if ans < 0 {
            -1
        } else {
            mask + ans as i32 * prev as i32 * 3
        }
    };
    let decode = |x: i32| -> (i32, i16) {
        if x < 0 {
            (0, i16::MIN / 2)
        } else {
            (x % (prev as i32 * 3), (x / (prev as i32 * 3)) as i16)
        }
    };
    let mut mem = Memoization3d::new(m + 1, n + 1, 3.power(n), |mem, col, row, mut mask| -> i32 {
        if col == m {
            for _ in 0..n {
                if mask % 3 == 2 {
                    return encode(0, i16::MIN / 2);
                }
                mask /= 3;
            }
            encode(0, 0)
        } else if row == n {
            encode(mask as i32, decode(mem.call(col + 1, 0, mask)).1)
        } else {
            if g[(row, col)] == b'#' {
                if mask % 3 == 2 {
                    encode(0, i16::MIN / 2)
                } else {
                    encode(
                        (mask / 3) as i32,
                        decode(mem.call(col, row + 1, mask / 3)).1,
                    )
                }
            } else {
                if mask % 3 == 2 {
                    if mask / prev == 2 && row != 0 {
                        mask -= 2 * prev;
                    }
                    encode(
                        (mask / 3 + prev) as i32,
                        decode(mem.call(col, row + 1, mask / 3 + prev)).1,
                    )
                } else {
                    let v1 = if mask / prev == 1 && row != 0 || mask % 3 == 1 {
                        (
                            (mask / 3) as i32,
                            decode(mem.call(col, row + 1, mask / 3)).1 + 1,
                        )
                    } else {
                        (
                            (mask / 3 + 2 * prev) as i32,
                            decode(mem.call(col, row + 1, mask / 3 + 2 * prev)).1 + 1,
                        )
                    };
                    if mask / prev == 2 && row != 0 {
                        mask -= 2 * prev;
                    }
                    let v2 = (
                        (mask / 3 + prev) as i32,
                        decode(mem.call(col, row + 1, mask / 3 + prev)).1,
                    );
                    let res = if v1.1 > v2.1 { v1 } else { v2 };
                    encode(res.0, res.1)
                }
            }
        }
    });
    let mut mask = 0;
    let mut row = 0;
    let mut col = 0;
    let mut ans = g.clone();
    while col != m {
        let (new_mask, _) = decode(mem.call(col, row, mask));
        let is_cooler = (new_mask as usize / prev) % 2;
        if g[(row, col)] == b'#' {
            assert_eq!(is_cooler, 0);
        } else {
            ans[(row, col)] = if is_cooler == 1 { b'1' } else { b'0' };
        }
        mask = new_mask as usize;
        if row == n - 1 {
            col += 1;
            row = 0;
        } else {
            row += 1;
        }
    }
    out.print_table(&ans);
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
