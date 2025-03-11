//{"name":"C. Binary Subsequence Value Sum","group":"Codeforces - Codeforces Round 1008 (Div. 1)","url":"https://mirror.codeforces.com/contest/2077/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 2\n010\n1\n3\n10 3\n0101000110\n3\n5\n10\n24 1\n011001100110000101111000\n24\n","output":"1\n5\n512\n768\n1536\n23068672\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let q = input.read_size();
    let mut s = input.read_str();

    type Mod = ModIntF;

    let mut count_ones = s.copy_count(b'1');
    let mut count_zeros = s.copy_count(b'0');

    #[derive(Default, Copy, Clone)]
    struct Node {
        ans: Mod,
        sum: Mod,
        qty_even: Mod,
        qty_odd: Mod,
    }

    impl Node {
        fn zero() -> Self {
            Self {
                ans: Mod::zero(),
                sum: Mod::one(),
                qty_even: Mod::one(),
                qty_odd: Mod::one(),
            }
        }

        fn one() -> Self {
            Self {
                ans: Mod::zero(),
                sum: -Mod::one(),
                qty_even: Mod::one(),
                qty_odd: Mod::one(),
            }
        }

        fn mult(a: Self, b: Self) -> Self {
            let res = Self {
                ans: a.ans * (b.qty_even + b.qty_odd)
                    + b.ans * (a.qty_even + a.qty_odd)
                    + a.sum * b.sum
                    + a.qty_odd * b.qty_odd,
                sum: a.sum * (b.qty_even + b.qty_odd) + b.sum * (a.qty_even + a.qty_odd),
                qty_even: a.qty_even * b.qty_even + a.qty_odd * b.qty_odd,
                qty_odd: a.qty_even * b.qty_odd + a.qty_odd * b.qty_even,
            };
            res
        }

        fn power(a: Self, n: usize) -> Self {
            if n == 0 {
                Self {
                    ans: Mod::zero(),
                    sum: Mod::zero(),
                    qty_even: Mod::one(),
                    qty_odd: Mod::zero(),
                }
            } else if n % 2 == 0 {
                let half = Self::power(a, n / 2);
                Self::mult(half, half)
            } else {
                Self::mult(a, Self::power(a, n - 1))
            }
        }
    }

    for _ in 0..q {
        let k = input.read_size() - 1;
        if s[k] == b'0' {
            count_zeros -= 1;
        } else {
            count_ones -= 1;
        }
        s[k] = if s[k] == b'0' { b'1' } else { b'0' };
        if s[k] == b'0' {
            count_zeros += 1;
        } else {
            count_ones += 1;
        }
        let ones = Node::power(Node::one(), count_ones);
        let zeroes = Node::power(Node::zero(), count_zeros);
        let ans = Node::mult(ones, zeroes).ans / Mod::new(2);
        out.print_line(ans);
    }
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
