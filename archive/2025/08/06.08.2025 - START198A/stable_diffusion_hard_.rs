//{"name":"Stable Diffusion (Hard)","group":"CodeChef - START198A","url":"https://www.codechef.com/START198A/problems/STDIF","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n5\n2 1 2 1 2\n6\n1 2 2 1 2 1\n","output":"3\n13\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::string_algorithms::palindromes::Palindromes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = n * (n + 1) / 2;
    let mut s = Str::new();
    let mut process = |s: Str| {
        let odd = s.odd_palindromes();
        let mut ones = Vec::with_capacity(s.len() + 1);
        let mut last = 0;
        ones.push(0);
        for c in s.copy_iter() {
            if c == b'1' {
                last += 1;
            }
            ones.push(last);
        }
        for i in s.indices() {
            if odd[i] <= 1 || s[i] == b'1' || s[i - 1] == b'1' || s[i + 1] == b'1' {
                continue;
            }
            let q_ones = ones[i] - ones[i + 1 - odd[i]];
            let delta = if s[i + 1 - odd[i]] == b'1' { 1 } else { 0 };
            ans -= odd[i] + delta - 1 - 2 * q_ones;
        }
    };
    for i in 1..n {
        if a[i] == a[i - 1] {
            s.push(b'1');
        } else {
            s.push(b'0');
        }
        if s.ends_with(b"11") || s.ends_with(b"101") {
            process(s);
            s = Str::new();
        }
    }
    process(s);
    out.print_line(ans);
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
