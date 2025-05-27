//{"name":"G. +++ and ××\\times with a sugar / +++, ××\\times 与糖","group":"Universal Cup - The 3rd Universal Cup. Stage 38: Guangzhou","url":"https://contest.ucup.ac/contest/2036/problem/11111","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n2 3 3 3\n5\n1 2 1 2 1\n6\n1 1 4 5 1 4\n","output":"54\n7\n82\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n).into_iter().collect::<VecDeque<_>>();

    let mut add = 0;
    while let Some(x) = a.back() {
        if *x == 1 {
            add += 1;
            a.pop_back();
        } else {
            break;
        }
    }
    while let Some(x) = a.front() {
        if *x == 1 {
            add += 1;
            a.pop_front();
        } else {
            break;
        }
    }
    if a.is_empty() {
        out.print_line(add);
        return;
    }
    let prod = a.copy_reduce(|x, y| x.saturating_mul(y)).unwrap();
    if prod >= 1_000_000 {
        type Mod = ModInt7;
        out.print_line(
            Mod::new_wide(add)
                + a.copy_map(|x| Mod::new_wide(x))
                    .reduce(|x, y| x * y)
                    .unwrap(),
        );
    } else {
        let bigs = a.copy_filter(|&x| x > 1).collect::<Vec<_>>();
        let ones = a
            .into_iter()
            .collect::<Vec<_>>()
            .split(|x| *x > 1)
            .map(|x| x.len() as i64)
            .collect::<Vec<_>>();
        let mut ans = 0;
        for i in usize::iter_all(bigs.len() - 1) {
            let mut cur = 1;
            let mut cand = add;
            for j in 0..bigs.len() - 1 {
                cur *= bigs[j];
                if i.is_set(j) {
                    cand += cur;
                    cand += ones[j + 1];
                    cur = 1;
                }
            }
            cur *= bigs[Back(0)];
            cand += cur;
            ans.maxim(cand);
        }
        out.print_line(ans);
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
