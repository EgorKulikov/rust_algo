//{"name":"day6","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::str::StrReader;
use algo_lib::string::trim::StrTrim;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut nums = Vec::new();
    loop {
        let next = input.peek().unwrap();
        if next != b' ' && !next.is_ascii_digit() {
            break;
        }
        nums.push(input.read_line());
    }
    let mut ops = Vec::new();
    while !input.is_empty() {
        ops.push(input.read_char());
    }
    let len = ops.len();
    let mut sep = Vec::new();
    for i in nums[0].indices() {
        let mut good = true;
        for j in nums.indices() {
            if nums[j][i] != b' ' {
                good = false;
                break;
            }
        }
        if good {
            sep.push(i);
        }
    }
    assert_eq!(sep.len() + 1, len);
    let table = Arr2d::with_gen(len, nums.len(), |col, row| {
        if col == 0 {
            &nums[row][..sep[0]]
        } else if col == len - 1 {
            &nums[row][sep[len - 2] + 1..]
        } else {
            &nums[row][sep[col - 1] + 1..sep[col]]
        }
    });

    let mut ans = 0;
    for i in 0..len {
        let it = table.row(i).map(|slice| {
            str_scan!(slice.trim(), "@", val: i64);
            val
        });
        let cur = match ops[i] {
            b'+' => it.sum::<i64>(),
            b'*' => it.product::<i64>(),
            _ => unreachable!(),
        };
        ans += cur;
    }
    out.print_line(ans);

    let mut ans2 = 0;
    let mut v = Vec::new();
    let mut at = 0;
    for i in 0.. {
        let mut is_sep = true;
        let mut cur = Vec::new();
        for j in nums.indices() {
            if i < nums[j].len() {
                cur.push(nums[j][i]);
                if nums[j][i] != b' ' {
                    is_sep = false;
                }
            }
        }
        if is_sep {
            ans2 += match ops[at] {
                b'+' => v.iter().sum::<i64>(),
                b'*' => v.iter().product::<i64>(),
                _ => unreachable!(),
            };
            v.clear();
            at += 1;
            if at == len {
                break;
            }
        } else {
            str_scan!(cur.as_slice().trim(), "@", val: i64);
            v.push(val);
        }
    }
    out.print_line(ans2);
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
