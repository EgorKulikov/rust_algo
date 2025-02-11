//{"name":"Worm Worries","group":"Kattis","url":"https://open.kattis.com/problems/worm","interactive":false,"timeLimit":3000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let _q = input.read_size();

    let mut query = Memoization::new(|_, c: [usize; 3]| -> i32 {
        out.print_line(('?', c));
        out.flush();
        input.read_int()
    });

    if m == 1 {
        let mut left = 1;
        let mut right = n;
        let mut mid = n * 3 / 5;
        let mut mid_val = query.call([mid, 1, 1]);
        while left < right {
            if mid - left > right - mid {
                let cand = (mid * 3 + left * 2) / 5;
                let cand_val = query.call([cand, 1, 1]);
                if cand_val < mid_val {
                    left = cand + 1;
                } else {
                    right = mid - 1;
                    mid = cand;
                    mid_val = cand_val;
                }
            } else {
                let cand = (mid * 3 + right * 2 + 4) / 5;
                let cand_val = query.call([cand, 1, 1]);
                if cand_val < mid_val {
                    right = cand - 1;
                } else {
                    left = mid + 1;
                    mid = cand;
                    mid_val = cand_val;
                }
            }
        }
        out.print_line(('!', mid, 1, 1));
        out.flush();
        return;
    }
    if k == 500 {
        let mut r = Random::new_with_seed(239);

        let mut best = 0;
        let mut best_at = [1, 1, 1];
        for _ in 0..10000 {
            let cand_at = [r.gen_range(1..=n), r.gen_range(1..=m), r.gen_range(1..=k)];
            let cand = query.call(cand_at);
            if cand > best {
                best = cand;
                best_at = cand_at;
            }
        }
        let limits = [n, m, k];
        loop {
            let mut found = false;
            for i in 0..3 {
                if best_at[i] > 1 {
                    let mut cand_at = best_at;
                    cand_at[i] -= 1;
                    let cand = query.call(cand_at);
                    if cand > best {
                        best = cand;
                        best_at = cand_at;
                        found = true;
                        break;
                    }
                }
                if best_at[i] < limits[i] {
                    let mut cand_at = best_at;
                    cand_at[i] += 1;
                    let cand = query.call(cand_at);
                    if cand > best {
                        best = cand;
                        best_at = cand_at;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                break;
            }
        }
        out.print_line(('!', best_at));
        return;
    }
    let mut left = [1, 1, 1];
    let mut right = [n, m, k];
    let mut last_max = 0;
    let mut last_max_at = [1, 1, 1];
    while left != right {
        let mut at = 0;
        for i in 1..3 {
            if right[i] - left[i] > right[at] - left[at] {
                at = i;
            }
        }
        let mid = (left[at] + right[at]) / 2;
        let mut from = left;
        from[at] = mid;
        let mut to = right;
        to[at] = mid;
        let mut max = query.call(from);
        let mut max_at = from;
        for i in from[0]..=to[0] {
            for j in from[1]..=to[1] {
                for l in from[2]..=to[2] {
                    let cand = query.call([i, j, l]);
                    if cand > max {
                        max = cand;
                        max_at = [i, j, l];
                    }
                }
            }
        }
        if last_max_at[at] == mid {
            let mut cc = last_max_at;
            cc[at] += 1;
            let cand = query.call(cc);
            if cand > last_max {
                last_max = cand;
                last_max_at = cc;
            }
        }
        max_at[at] += 1;
        let cand = query.call(max_at);
        if max.max(cand) <= last_max {
            if last_max_at[at] <= mid {
                right[at] = mid;
            } else {
                left[at] = mid + 1;
            }
            continue;
        }
        if cand > max {
            left[at] = mid + 1;
            last_max = cand;
            last_max_at = max_at;
        } else {
            right[at] = mid;
            last_max = max;
            max_at[at] -= 1;
            last_max_at = max_at;
        }
    }
    out.print_line(('!', left));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
//END MAIN
