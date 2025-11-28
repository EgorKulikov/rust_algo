//{"name":"E. Binary Strings and Blocks","group":"Codeforces - Educational Codeforces Round 185 (Rated for Div. 2)","url":"https://codeforces.com/contest/2170/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 3\n1 2\n2 3\n3 4\n4 2\n1 2\n3 4\n200 1\n13 37\n","output":"2\n4\n570529459\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_utils::powers;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut lr = input.read_size_pair_vec(m).dec();

    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));
    let mut cur_start = lr[0].0;
    let mut s = vec![cur_start];
    let mut e = vec![lr[0].1 + 1];
    for (l, r) in lr {
        if cur_start < l {
            s.push(l);
            e.push(r + 1);
            cur_start = l;
        }
    }

    s.push(n);

    let pw = powers(Mod::new(2), n + 1);
    type Mod = ModIntF;
    let mut mem = Memoization2d::new(2, e.len() + 1, |mem, mode, pos| -> (Mod, Mod) {
        if pos == e.len() {
            if mode == 0 {
                (Mod::one(), Mod::zero())
            } else {
                (Mod::zero(), Mod::zero())
            }
        } else if mode == 0 {
            let not_take = mem.call(0, pos + 1).0 * pw[s[pos + 1] - s[pos]];
            let take = mem.call(1, pos).0 * 2;
            (not_take - take, Mod::zero())
        } else {
            let at = s.lower_bound(&e[pos]);
            let mut res = mem.call(0, at).0 * pw[s[at] - e[pos]];
            res -= mem.call(1, pos + 1).1;
            res += mem.call(1, at).1;
            // mem.call(0, at) * pw[s[at] - e[pos]] - mem.call(1, pos + 1)
            (res, res + mem.call(1, pos + 1).1)
        }
    });
    out.print_line(mem.call(0, 0).0 * pw[s[0]]);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
