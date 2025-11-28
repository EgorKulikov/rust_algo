//{"name":"D. Almost Roman","group":"Codeforces - Educational Codeforces Round 185 (Rated for Div. 2)","url":"https://codeforces.com/contest/2170/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 3\n???\n3 0 0\n2 3 1\n0 1 2\n10 7\n??IV?VXIV?\n0 0 4\n4 4 0\n1 1 2\n1 1 3\n1 1 4\n1 2 1\n2 2 0\n9 5\n?V????IVV\n9 2 4\n4 1 5\n0 1 4\n4 8 1\n3 2 7\n3 2\nI?V\n0 1 0\n0 0 1\n","output":"30\n9\n5\n25\n43\n36\n27\n25\n42\n53\n19\n17\n19\n33\n17\n9\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut s = input.read_str();

    let mut base = 0i32;
    let wildcards = s.copy_count(b'?');
    for i in 0..n {
        match s[i] {
            b'I' | b'?' => {
                if i + 1 < n && (s[i + 1] == b'V' || s[i + 1] == b'X') {
                    base -= 1;
                } else {
                    base += 1;
                }
            }
            b'V' => base += 5,
            b'X' => base += 10,
            _ => unreachable!(),
        }
    }
    let mut res = vec![base];
    for i in (0..n).rev() {
        if s[i] == b'?' {
            if i > 0
                && (s[i - 1] == b'I' || s[i - 1] == b'?')
                && (i + 1 == n || s[i + 1] != b'V' && s[i + 1] != b'X')
            {
                base += 2;
                res.push(base);
                s[i] = b'V';
            }
        }
    }
    for i in (0..n).rev() {
        if s[i] == b'?' {
            if i > 0 && (s[i - 1] == b'I' || s[i - 1] == b'?')
                || (i + 1 == n || s[i + 1] != b'V' && s[i + 1] != b'X')
            {
                base += 4;
                res.push(base);
                s[i] = b'V';
            }
        }
    }
    for i in (0..n).rev() {
        if s[i] == b'?' {
            base += 6;
            res.push(base);
            s[i] = b'V';
        }
    }

    for _ in 0..q {
        let _cx = input.read_size();
        let cv = input.read_size();
        let ci = input.read_size();

        let need_v = wildcards.saturating_sub(ci);
        let need_x = wildcards.saturating_sub(ci + cv);
        out.print_line(res[need_v] + 5 * need_x as i32);
    }
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
