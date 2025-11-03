//{"name":"C. Crossing River","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14708","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5 2\n2 1 13 19 11\n12 18 19 7 8\n","output":"25\n5 0 2\n7 1 4\n9 0 1\n11 1 5\n13 0 5\n15 1 1\n17 0 3\n19 1 2\n21 0 4\n23 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);
    let mut b = input.read_long_vec(m);

    let mut ans = None;
    let mut schedule = Vec::new();
    for t in 0..2 {
        let mut time = -k;
        let mut cur = Vec::new();
        let a_order = (0..a.len()).collect::<Vec<_>>().sorted_by_key(|&i| a[i]);
        let b_order = (0..b.len()).collect::<Vec<_>>().sorted_by_key(|&i| b[i]);
        for i in (0..n.max(m)).rev() {
            if i < a.len() {
                time.maxim(a[a_order[Back(i)]]);
                cur.push((time, t, a_order[Back(i)] + 1));
            }
            time += k;
            if i < b.len() {
                time.maxim(b[b_order[Back(i)]]);
                cur.push((time, 1 - t, b_order[Back(i)] + 1));
            }
            time += k;
        }
        if ans.minim(time) {
            schedule = cur;
        }
        swap(&mut a, &mut b);
    }
    out.print_line(ans);
    out.print_per_line(&schedule);
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
