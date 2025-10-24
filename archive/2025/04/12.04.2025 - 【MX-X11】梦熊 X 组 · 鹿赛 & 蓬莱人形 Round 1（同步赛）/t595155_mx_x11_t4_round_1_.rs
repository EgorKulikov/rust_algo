//{"name":"T595155 【MX-X11-T4】「蓬莱人形 Round 1」视奸","group":"Luogu","url":"https://www.luogu.com.cn/problem/T595155?contestId=240581","interactive":false,"timeLimit":2000,"tests":[{"input":"0 2\n9\n100110011\n3 0 -1 -3 4 -1 -4 -3 -5\n8\n10100101\n2 0 2 4 1 1 2 2\n","output":"-4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let p = input.read_long_vec(n);

    let mut start = 0;
    let mut end = 0;
    let mut ans = 0;
    let mut process = |start: usize, end: usize| {
        if s[start..end].iter().all(|c| *c == b'1') {
            ans += p[start..end].iter().sum::<i64>();
            return;
        }
        let mut res = 0;
        let mut min_inner = i64::MAX;
        for i in start..end {
            if p[i] <= 0 {
                res += p[i];
            }
            if i > start && i + 1 < end {
                min_inner.minim(p[i]);
            }
        }
        if min_inner > 0 {
            res += min_inner;
        }
        if end - start == 3 && s[start + 1] == b'0' {
            res.minim(p[start] + p[end - 1]);
        }
        ans += res;
    };
    for i in 0..n {
        if s[i] == b'1' {
            if start == end {
                start = i;
            }
            end = i + 1;
        } else if i > end && start != end {
            process(start, end);
            start = 0;
            end = 0;
        }
    }
    if start != end {
        process(start, end);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    input.read_int();

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
