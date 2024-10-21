//{"name":"F. Zeroth Rome","group":"Yandex - Yandex Cup 2024 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/69390/problems/F/","interactive":true,"timeLimit":1000,"tests":[{"input":"1\n\n\n\n\n1 0 1\n","output":"3\n1 2 3 4\n5 6 7 8\n9 10 11 12\n\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FZerothRome"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashSet;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();

    const K: usize = 15;
    const MAX: usize = 2024;
    let mut next = 0;
    let mut requests = vec![Vec::new(); K];
    let mut bad = HashSet::new();
    for i in usize::iter_all(K) {
        let mut ok = !bad.contains(&i);
        for j in 0..K {
            if bad.contains(&i.flipped_bit(j)) {
                ok = false;
            }
        }
        if !ok {
            continue;
        }
        for j in 0..K {
            if i.is_set(j) {
                requests[j].push(next);
            }
        }
        next += 1;
        if next == MAX + 1 {
            break;
        }
        bad.insert(i);
        for j in 0..K {
            bad.insert(i.flipped_bit(j));
        }
    }
    assert_eq!(next, MAX + 1);
    out.print_line(K);
    out.print_per_line(&requests);
    out.flush();

    input.skip_whitespace();
    if input.peek() == Some(b'-') {
        return;
    }
    let ans = input.read_int_table(t, K);
    for i in 0..t {
        let mut found = false;
        for res in 0..=MAX {
            let mut bad = 0;
            for j in 0..K {
                if requests[j].bin_search(&res).is_some() != (ans[(i, j)] == 1) {
                    bad += 1;
                }
            }
            if bad <= 1 {
                out.print_line(res);
                out.flush();
                found = true;
                break;
            }
        }
        if !found {
            loop {}
        }
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
