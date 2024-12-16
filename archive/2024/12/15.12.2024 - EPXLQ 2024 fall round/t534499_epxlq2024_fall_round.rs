//{"name":"T534499 [EPXLQ2024 fall round] 好排列","group":"Luogu","url":"https://www.luogu.com.cn/problem/T534499?contestId=201084","interactive":false,"timeLimit":1500,"tests":[{"input":"2\n3 1\n6 4\n","output":"1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T534499EPXLQ2024FallRound"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if k == 1 {
        out.print_line(1);
        return;
    }
    if k == 2 {
        out.print_line(n);
        return;
    }
    let mut to_left = vec![0; n + 1];
    let mut to_right = vec![0; n + 1];
    to_left[1] = 1;
    to_right[1] = 1;
    to_left[n] = 1;
    to_right[n] = 1;
    let mut segs = BinaryHeap::new();
    fn make_odd(n: usize) -> usize {
        if n % 2 == 0 {
            n - 1
        } else {
            n
        }
    }
    segs.push((make_odd(n - 2), Reverse(2), n - 2));
    let mut next = 3;
    while let Some((_, Reverse(start), len)) = segs.pop() {
        if len < 3 {
            segs.push((0, Reverse(start), len));
            break;
        }
        let mid = start + (len - 1) / 2;
        to_left[mid] = 1;
        to_right[mid] = 1;
        if next == k {
            out.print_line(mid);
            return;
        }
        next += 1;
        segs.push((make_odd(mid - start), Reverse(start), mid - start));
        segs.push((
            make_odd(start + len - mid - 1),
            Reverse(mid + 1),
            start + len - mid - 1,
        ));
    }
    let mut set = BTreeSet::new();
    for (_, Reverse(start), _) in segs {
        set.insert((to_left[start - 1] + to_right[start + 1], start));
    }
    while let Some(&(val, pos)) = set.iter().next() {
        if next == k {
            out.print_line(pos);
            return;
        }
        next += 1;
        set.remove(&(val, pos));
        let left = pos - 1 - to_left[pos - 1];
        let right = pos + 1 + to_right[pos + 1];
        if left != 0 {
            set.remove(&(to_left[left - 1] + to_right[left + 1], left));
            to_right[left + 1] = right - left - 1;
            if to_left[left - 1] != 0 {
                set.insert((to_left[left - 1] + to_right[left + 1], left));
            }
        }
        if right != n + 1 {
            set.remove(&(to_left[right - 1] + to_right[right + 1], right));
            to_left[right - 1] = right - left - 1;
            set.insert((to_left[right - 1] + to_right[right + 1], right));
        }
    }
    unreachable!();
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
