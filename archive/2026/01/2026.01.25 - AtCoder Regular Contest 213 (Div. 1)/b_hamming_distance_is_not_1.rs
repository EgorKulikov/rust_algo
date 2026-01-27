//{"name":"B - Hamming Distance is not 1","group":"AtCoder - AtCoder Regular Contest 213 (Div. 1)","url":"https://atcoder.jp/contests/arc213/tasks/arc213_b","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n1 0 3\n0 1 2\n1 213 213\n","output":"1001\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_iterator::iterate_with_base;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_int();
    let l = input.read_long();
    let r = input.read_long();

    fn best_count(l: i64, r: i64) -> i64 {
        let mut even = 0;
        let mut odd = 0;
        for (prefix, len, _) in iterate_with_base(l, r, 2) {
            if len == 0 {
                if prefix.count_ones() % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            } else {
                even += 1 << (len - 1);
                odd += 1 << (len - 1);
            }
        }
        even.max(odd)
    }

    fn best_arr(l: i64, r: i64) -> Str {
        let mut even = Str::new();
        let mut odd = Str::new();
        for i in l..=r {
            if i.count_ones() % 2 == 0 {
                even.push(b'1');
                odd.push(b'0');
            } else {
                even.push(b'0');
                odd.push(b'1');
            }
        }
        if even.copy_count(b'1') >= odd.copy_count(b'1') {
            even
        } else {
            odd
        }
    }

    fn solve_segment(q: i32, l: i64, r: i64) -> Result<i64, Str> {
        if q == 0 {
            Ok(best_count(l, r))
        } else {
            Err(best_arr(l, r))
        }
    }

    fn solve(q: i32, l: i64, r: i64) -> Result<i64, Str> {
        if l == 0 {
            return solve_segment(q, l, r);
        }
        if l.highest_bit() != r.highest_bit() {
            if r - (1 << r.highest_bit()) >= l {
                solve_segment(q, l, r)
            } else {
                let front = solve(q, l, (1 << r.highest_bit()) - 1);
                let back = solve(q, 1 << r.highest_bit(), r);
                if q == 0 {
                    unsafe { Ok(front.unwrap_unchecked() + back.unwrap_unchecked()) }
                } else {
                    let mut res = front.err().unwrap();
                    res += &back.err().unwrap();
                    Err(res)
                }
            }
        } else {
            solve(q, l - (1 << l.highest_bit()), r - (1 << r.highest_bit()))
        }
    }

    let ans = solve(q, l, r);
    match ans {
        Ok(a) => out.print_line(a),
        Err(a) => out.print_line(a),
    }
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
