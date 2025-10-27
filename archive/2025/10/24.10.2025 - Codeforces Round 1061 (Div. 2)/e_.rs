//{"name":"E. Лучшее время для покупки и продажи акций","group":"Codeforces - Codeforces Round 1061 (Div. 2)","url":"https://codeforces.com/contest/2156/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n5\n5 1 2 3 4\n4\n3 1 2 1\n10\n7 1 3 5 8 2 8 3 5 1\n6\n1 1 4 5 1 4\n9\n9 9 8 2 4 4 3 5 3\n4\n1000000000 1 2 3\n","output":"1\n-2\n5\n3\n1\n-999999998\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut left = -999_999_999;
    let mut right = 999_999_999;
    let mut bad = vec![Vec::new(); n];
    while left < right {
        let mid = left + (right - left + 1) / 2;
        bad.fill(Vec::new());
        // let mut more_than_2 = Vec::new();
        let mut set = BTreeSet::new();

        for i in 0..n {
            let mut x = set.floor(&((a[i] - mid, i))).copied();
            while let Some(val) = x {
                if bad[i].len() > 2 {
                    break;
                }
                bad[i].push(val.1);
                x = set.prev(&val).copied();
            }
            set.insert((a[i], i));
        }
        set.clear();
        for i in (0..n).rev() {
            let mut x = set.ceil(&((a[i] + mid, i))).copied();
            while let Some(val) = x {
                if bad[i].len() > 2 {
                    break;
                }
                bad[i].push(val.1);
                x = set.next(&val).copied();
            }
            set.insert((a[i], i));
        }
        let mut more_than_2 = Vec::new();
        let mut twos = Vec::new();
        for i in 0..n {
            if bad[i].len() > 2 {
                more_than_2.push(i);
            } else if bad[i].len() == 2 {
                twos.push((i, bad[i][0], bad[i][1]));
            }
        }
        more_than_2.sort();
        more_than_2.dedup();
        if more_than_2.len() > 1 {
            left = mid;
            continue;
        }
        if twos.is_empty() {
            right = mid - 1;
            continue;
        }
        let mut cand = if more_than_2.len() == 1 {
            vec![more_than_2[0]]
        } else {
            vec![twos[0].0, twos[0].1, twos[0].2]
        };
        for (a, b, c) in twos {
            let mut new_cand = Vec::new();
            for &x in &cand {
                if x == a || x == b || x == c {
                    new_cand.push(x);
                }
            }
            cand = new_cand;
        }
        if !cand.is_empty() {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    out.print_line(left);
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
