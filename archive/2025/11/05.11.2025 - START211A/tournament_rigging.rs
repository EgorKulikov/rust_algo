//{"name":"Tournament Rigging","group":"CodeChef - START211A","url":"https://www.codechef.com/START211A/problems/KUPSET","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 1 2\n2 2 1\n3 2 4\n4 6 11\n","output":"2 3 2 3 4 2 1\n-1\n4 5 4 6 5 1 4 8 6 5 7 2 1 3 4\n11 11 14 11 4 5 14 11 12 7 4 8 5 14 10 11 16 12 1 15 7 6 4 9 8 13 5 2 14 10 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let w = input.read_size();

    let inv = k >= (1 << (n - 1));

    let build = |w: usize, k: usize| {
        let mut ans = vec![0; (1 << (n + 1)) - 1];
        let mut at = 1;
        ans[Back(0)] = w;
        for i in (1..=(1 << n)).rev() {
            if i != w {
                ans[Back(at)] = i;
                at += 1;
            }
        }
        let mut num_upsets = 0;
        for i in (0..(1 << n) - 1).rev() {
            if ans[2 * i + 2] == w {
                if ans[2 * i + 1] < w {
                    num_upsets += 1;
                }
                ans[i] = w;
            } else {
                ans[i] = ans[2 * i + 1].min(ans[2 * i + 2]);
            }
        }
        if num_upsets > k {
            return None;
        }
        let mut rem = k - num_upsets;
        for i in 1..1 << (n - 1) {
            let a1 = ans[Back(2 * i)];
            let a2 = ans[Back(2 * i + 1)];
            if a1 == w - 1 && a2 == w + 1 || a1 == w + 1 && a2 == w - 1 {
                continue;
            }
            if rem > 0 {
                rem -= 1;
                let mut pos = (1 << (n + 1)) - 2 - i * 2;
                while pos > 0 {
                    pos -= 1;
                    pos /= 2;
                    if ans[pos] == a1 || ans[pos] == a2 {
                        ans[pos] = a1 + a2 - ans[pos];
                    } else {
                        break;
                    }
                }
            }
        }
        Some(ans)
    };
    let ans = if inv {
        build((1 << n) + 1 - w, (1 << n) - k - 1)
    } else {
        build(w, k)
    };
    if let Some(mut ans) = ans {
        if inv {
            for x in &mut ans {
                *x = (1 << n) + 1 - *x;
            }
        }
        out.print_line(ans);
    } else {
        out.print_line(-1);
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
