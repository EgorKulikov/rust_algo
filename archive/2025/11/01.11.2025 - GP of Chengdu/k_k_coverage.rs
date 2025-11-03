//{"name":"K. K-Coverage","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14716","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2 1\n2 6 2\n3 3 2\n6 2 0\n5 1 3\n5 6 7 8 9\n","output":"6\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let k = input.read_int();
    let ll = input.read_size_vec(n);

    let len = 2 * n + 2 * l + 1;
    let mut start = vec![0i32; len];
    for x in ll.copy_iter() {
        start[x] += 1;
    }
    let mut qty = vec![0; len];
    let mut cur = 0;
    for i in 0..len {
        cur += start[i];
        if i >= l {
            cur -= start[i - l];
        }
        qty[i] = cur;
    }
    let base_ans = qty.copy_count(k);
    let mut max_delta = 0;
    let mut set = BTreeSet::new();
    let mut base = 0;
    set.insert((0, 0));
    let mut v = vec![0];
    for i in 0..len - l {
        if qty[i] == k {
            base -= 1;
        }
        if qty[i] == k - 1 {
            base += 1;
        }
        if qty[i + l] == k {
            base -= 1;
        }
        if qty[i + l] == k + 1 {
            base += 1;
        }
        set.insert((base, i + 1));
        v.push(base);
        if i >= l - 1 {
            set.remove(&(v[i - (l - 1)], i - (l - 1)));
        }
        let &(min_val, _) = set.iter().next().unwrap();
        if start[i + 1] != 0 {
            max_delta.maxim(base - min_val);
        }
    }
    let mut set = BTreeSet::new();
    let mut base = 0;
    set.insert((0, len - l));
    let mut v = vec![0; len - l + 1];
    for i in (0..len - l).rev() {
        if qty[i] == k {
            base -= 1;
        }
        if qty[i] == k + 1 {
            base += 1;
        }
        if qty[i + l] == k {
            base -= 1;
        }
        if qty[i + l] == k - 1 {
            base += 1;
        }
        set.insert((base, i));
        v[i] = base;
        if i + l <= len - l {
            set.remove(&(v[i + l], i + l));
        }
        let &(min_val, _) = set.iter().next().unwrap();
        if start[i] != 0 {
            max_delta.maxim(base - min_val);
        }
    }
    let mut remove = Vec::new();
    let mut add = Vec::new();
    let mut cur_remove = 0;
    let mut cur_add = 0;
    for i in 0..len {
        if qty[i] == k {
            cur_remove -= 1;
            cur_add -= 1;
        }
        if qty[i] == k - 1 {
            cur_add += 1;
        }
        if qty[i] == k + 1 {
            cur_remove += 1;
        }
        if i >= l {
            if qty[i - l] == k {
                cur_remove += 1;
                cur_add += 1;
            }
            if qty[i - l] == k - 1 {
                cur_add -= 1;
            }
            if qty[i - l] == k + 1 {
                cur_remove -= 1;
            }
        }
        if i >= l - 1 {
            remove.push(cur_remove);
            add.push(cur_add);
        }
    }
    let mut best_remove = i32::MIN / 2;
    let mut best_add = i32::MIN / 2;
    for i in remove.indices() {
        if start[i] != 0 {
            best_remove.maxim(remove[i]);
        }
        best_add.maxim(add[i]);
        if i + l < remove.len() {
            max_delta.maxim(add[i + l] + best_remove);
            if start[i + l] != 0 {
                max_delta.maxim(remove[i + l] + best_add);
            }
        }
    }
    out.print_line(base_ans as i32 + max_delta);
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
