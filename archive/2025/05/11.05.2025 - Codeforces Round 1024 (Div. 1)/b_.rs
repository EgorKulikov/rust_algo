//{"name":"B. Перестановка квартетов","group":"Codeforces - Codeforces Round 1024 (Div. 1)","url":"https://codeforces.com/contest/2101/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n3 4 1 2\n5\n5 4 3 1 2\n10\n10 9 8 7 6 5 4 3 2 1\n","output":"1 2 3 4\n2 1 3 4 5\n2 1 4 3 6 5 8 7 10 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::interleave::interleave;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    fn is_odd(a: &[usize]) -> bool {
        let mut res = false;
        let mut ft = FenwickTree::new(a.len());
        for i in a.indices() {
            if ft.get(a[i]..) % 2 == 1 {
                res = !res;
            }
            ft.add(a[i], 1);
        }
        res
    }
    let was_odd = is_odd(&a);
    let even = a.copy_step_by(2).collect::<Vec<_>>().sorted();
    let odd = a.copy_skip(1).step_by(2).collect::<Vec<_>>().sorted();
    let mut ans = interleave(even.into_iter(), odd.into_iter()).collect::<Vec<_>>();
    if is_odd(&ans) != was_odd {
        ans.swap(n - 3, n - 1);
    }
    out.print_line(ans.inc());
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
