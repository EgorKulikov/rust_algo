//{"name":"#exclude&lt;scoring&gt;","group":"Kattis","url":"https://open.kattis.com/problems/excludescoring","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n50 50 75\n25 25 25\n","output":"2\n"},{"input":"5 2\n50 50 50 50\n25 25 25 25\n","output":"1\n\n\n\n"},{"input":"2 4\n90\n1\n3\n2\n","output":"3\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::{PartialSums, UpperDiv};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_int_table(m, n - 1);

    let score = s
        .rows()
        .map(|r| {
            let cur = s.row(r).copied().collect::<Vec<_>>().sorted();
            let mut sum3 = 0;
            let mut sum4 = 0;
            for i in 0..4 {
                if i + 1 < n {
                    sum4 += cur[Back(i)];
                    if i < 3 {
                        sum3 += cur[Back(i)];
                    }
                }
            }
            (sum3, sum4)
        })
        .collect::<Vec<_>>();

    let our_score = score[0].1;
    let mut place = 1;
    let mut other = Vec::new();
    for (s3, s4) in score.iter_skip(1) {
        if s4 > our_score || s3 + 1 > our_score {
            place += 1;
        } else {
            other.push(s3);
        }
    }
    other.sort();
    other.reverse();
    let add = [
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 21, 23, 25, 27, 30, 33, 37, 41,
        46, 51, 61, 76, 101,
    ];
    let mut a = vec![1; other.len()];
    for i in 0..other.len().min(add.len()) {
        a[Back(i)] = add[Back(i)];
    }
    let s_add = a.partial_sums();
    let mut left = 0;
    let mut right = other.len();
    while left < right {
        let i = (left + right + 1) / 2;
        let mut cur = 0;
        let mut cur_len = 0;
        let mut start_len = 0;
        let mut good = true;
        for j in 0..i {
            if cur_len == 0 {
                cur = 0;
                start_len = 0;
            }
            if other[j] + cur <= our_score {
                let base = j + cur_len - start_len;
                let mut left = start_len + 1;
                let mut right = i - base + 1;
                while left < right {
                    let k = (left + right) / 2;
                    cur = (s_add[Back(i - (base + k))] - s_add[Back(i - base)]).upper_div(k as i32);
                    if other[j] + cur > our_score {
                        right = k;
                    } else {
                        left = k + 1;
                    }
                }
                if left == i - base + 1 {
                    good = false;
                    break;
                }
                let k = left;
                cur = (s_add[Back(i - (base + k))] - s_add[Back(i - base)]).upper_div(k as i32);
                cur_len += k - start_len;
                start_len = k;
            }
            cur_len -= 1;
        }
        if good {
            left = i;
        } else {
            right = i - 1;
        }
    }
    out.print_line(place + left);
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
