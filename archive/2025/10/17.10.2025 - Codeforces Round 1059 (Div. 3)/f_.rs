//{"name":"F. Красивые отрезки","group":"Codeforces - Codeforces Round 1059 (Div. 3)","url":"https://codeforces.com/contest/2162/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1\n1 2\n3 5\n1 1\n1 2\n2 2\n2 2\n2 3\n4 5\n1 2\n2 3\n3 4\n1 1\n4 4\n5 4\n3 5\n1 1\n2 4\n4 4\n4 2\n1 3\n2 4\n","output":"2 0 1\n2 1 0\n0 2 1 3\n2 0 1 3 4\n3 1 0 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let lr = input.read_size_pair_vec(m).dec();

    let (l, r) = lr.detuple();

    let l_max = l.copy_max();
    let r_min = r.copy_min();
    let mut ans = (0..n).collect::<Vec<_>>();
    if l_max <= r_min {
        ans.swap(0, l_max);
        out.print_line(ans);
        return;
    }
    for i in 0..n - 1 {
        if !l.contains(&(i + 1)) {
            ans.swap(0, i + 1);
            if i != 0 {
                ans.swap(1, i);
            }
            out.print_line(ans);
            return;
        }
        if !r.contains(&i) {
            ans.swap(1, i + 1);
            ans.swap(0, i);
            out.print_line(ans);
            return;
        }
    }
    ans.swap(1, n - 1);
    out.print_line(ans);
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
