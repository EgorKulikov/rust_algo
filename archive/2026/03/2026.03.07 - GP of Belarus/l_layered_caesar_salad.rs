//{"name":"L. Layered Caesar Salad","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17271","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 5\ndelta\nalpha\nalpha\nprime\n1 20\npetrozavodskprogcamp\n","output":"alpha\ndelta\nprime\nalpha\npetrozavodskprogcamp\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_map::ArrayMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str_vec(n);

    let mut present: i32 = 0;
    for i in 0..n {
        let cur = s[i][0] as usize - b'a' as usize;
        present.set_bit(cur);
    }
    let mut non_present = 0;
    while present.is_set(non_present) {
        non_present += 1;
    }
    let shift = if present.count_ones() == 1 {
        vec![1; n]
    } else {
        Vec::with_gen(n, |i| {
            let cur = s[i][0] as usize - b'a' as usize;
            if cur <= non_present {
                non_present - cur
            } else {
                26 + non_present - cur
            }
        })
    };
    for i in 0..n {
        let mut add = 0;
        for j in 0..k {
            let mut cur = s[i][j] as usize - b'a' as usize;
            cur += shift[i] + add;
            cur %= 26;
            add = cur;
            out.print(b'a' + cur as u8);
        }
        out.print_line(());
    }
}

fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _k = input.read_size();
    let s = input.read_str_vec(2 * n);

    let mut by_first = ArrayMap::<_, Vec<_>>::new(b'a'..=b'z');
    for i in 0..2 * n {
        by_first[s[i][0]].push(i);
    }
    let mut full = Vec::new();
    for c in b'a'..=b'z' {
        if by_first[c].len() == n {
            full.push(c);
        }
    }
    let skip = if full.len() == 1 {
        full[0]
    } else if full[1] == full[0] + 1 {
        full[1]
    } else {
        assert_eq!(full[0], b'a');
        assert_eq!(full[1], b'z');
        b'a'
    };
    for c in b'a'..=b'z' {
        if c == skip {
            continue;
        }
        for i in by_first[c].copy_iter() {
            out.print_line(&s[i]);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::RunTwiceMultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
        TestType::RunTwiceMultiNumber => {
            let s = input.read_str();
            let is_encode = s.as_slice() == b"ENCODE";
            let t = input.read();
            if is_encode {
                for i in 1..=t {
                    solve(&mut input, &mut output, i, &mut pre_calc);
                }
            } else {
                for i in 1..=t {
                    solve2(&mut input, &mut output, i, &mut pre_calc);
                }
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
