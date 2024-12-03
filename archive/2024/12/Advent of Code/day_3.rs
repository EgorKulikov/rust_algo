//{"name":"day_3","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_3"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    // part 1
    {
        let mut ans = 0;
        for s in &data {
            for i in s.indices() {
                if s[i..].starts_with(b"mul(") {
                    let mut at = i + 4;
                    let mut left = 0;
                    while at < s.len() && s[at].is_ascii_digit() && at < i + 7 {
                        left *= 10;
                        left += (s[at] - b'0') as usize;
                        at += 1;
                    }
                    if at == s.len() || s[at] != b',' {
                        continue;
                    }
                    at += 1;
                    let mut right = 0;
                    let start = at;
                    while at < s.len() && s[at].is_ascii_digit() && at < start + 3 {
                        right *= 10;
                        right += (s[at] - b'0') as usize;
                        at += 1;
                    }
                    if at == s.len() || s[at] != b')' {
                        continue;
                    }
                    ans += left * right;
                }
            }
        }
        out.print_line(ans);
    }
    // part 2
    {
        let mut ans = 0;
        let mut enabled = true;
        for s in &data {
            for i in s.indices() {
                if s[i..].starts_with(b"do()") {
                    enabled = true;
                }
                if s[i..].starts_with(b"don't()") {
                    enabled = false;
                }
                if enabled && s[i..].starts_with(b"mul(") {
                    let mut at = i + 4;
                    let mut left = 0;
                    while at < s.len() && s[at].is_ascii_digit() && at < i + 7 {
                        left *= 10;
                        left += (s[at] - b'0') as usize;
                        at += 1;
                    }
                    if at == s.len() || s[at] != b',' {
                        continue;
                    }
                    at += 1;
                    let mut right = 0;
                    let start = at;
                    while at < s.len() && s[at].is_ascii_digit() && at < start + 3 {
                        right *= 10;
                        right += (s[at] - b'0') as usize;
                        at += 1;
                    }
                    if at == s.len() || s[at] != b')' {
                        continue;
                    }
                    ans += left * right;
                }
            }
        }
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
