//{"name":"B1. Unique Values (Easy version)","group":"Codeforces - Codeforces Round 1093 (Div. 1)","url":"https://codeforces.com/contest/2219/problem/B1","interactive":true,"timeLimit":2000,"tests":[{"input":"1\n2\n\n0\n\n2\n\n2\n\n0\n\n1\n","output":"\n\n? 2 1 2\n\n? 2 1 4\n\n? 2 1 5\n\n? 5 1 2 3 4 5\n\n? 4 1 2 3 4\n\n! 1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, Shuffle};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut mem = FxHashMap::default();
    let mut query = |q: &[usize]| -> usize {
        if q.len() <= 1 {
            return q.len();
        }
        if let Some(val) = mem.get(q) {
            return *val;
        }
        out.print_line((b'?', q.len(), q));
        out.flush();
        let res = input.read_size();
        mem.insert(q.to_vec(), res);
        res
    };

    let mut r = Random::new_with_seed(239);
    let mut order = (1..=2 * n + 1).collect::<Vec<_>>();
    loop {
        order.shuffle_with(&mut r);
        let left = query(&order[..n]);
        let right = query(&order[n..]);
        if left == right {
            continue;
        }
        let mut l0 = 0;
        let mut r1 = 2 * n;
        let mut l1 = n;
        let mut r0 = n - 1;
        let mm;
        if left < right {
            let mut ll = 0;
            let mut rr = n - 1;
            while ll < rr {
                let mid = (ll + rr + 1) / 2;
                let left = query(&order[..mid]);
                let right = query(&order[mid..]);
                if left < right {
                    rr = mid - 1;
                    r0 = mid - 1;
                } else {
                    ll = mid;
                    if left == right {
                        l0 = mid;
                        ll += 1;
                    }
                }
            }
            mm = ll;
        } else {
            let mut ll = n;
            let mut rr = 2 * n;
            while ll < rr {
                let mid = (ll + rr + 1) / 2;
                let left = query(&order[..mid]);
                let right = query(&order[mid..]);
                if left > right {
                    ll = mid;
                    l1 = mid;
                } else {
                    rr = mid - 1;
                    if left == right {
                        r1 = mid;
                        rr -= 1;
                    }
                }
            }
            mm = ll;
        }
        while l0 < r0 {
            let mid = (l0 + r0 + 1) / 2;
            let left = query(&order[..mid]);
            let right = query(&order[mid..]);
            if left == right {
                l0 = mid;
            } else {
                r0 = mid - 1;
            }
        }
        while l1 < r1 {
            let mid = (l1 + r1 + 1) / 2;
            let left = query(&order[..mid]);
            let right = query(&order[mid..]);
            if left == right {
                r1 = mid - 1;
            } else {
                l1 = mid;
            }
        }
        out.print_line(('!', order[l0], order[mm], order[l1]));
        out.flush();
        return;
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
