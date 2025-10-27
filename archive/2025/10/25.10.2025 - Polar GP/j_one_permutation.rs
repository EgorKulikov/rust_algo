//{"name":"J. One Permutation","group":"Universal Cup - Polar GP","url":"https://contest.ucup.ac/contest/2559/problem/14424","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 4 5\n5\n5 4 3 2 1\n8\n2 4 1 3 6 5 8 7\n9\n3 2 1 6 5 4 9 8 7\n1\n1\n","output":"5 5 5 5 5\n1 2 3 4 5\n4 6 7 8 8 8 8 8\n3 4 5 6 7 8 9 9 9\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let last = if i == 0 { 0 } else { v[i - 1] };
        let mut cur = p[i] + last / n * n;
        if cur < last {
            cur += n;
        }
        v.push(cur);
        for j in (0..v.len() - 1).rev() {
            if v[j] + n < cur {
                cur -= n;
                v[j + 1] = cur;
            }
        }
        v[0].minim(p[i]);
    }
    let mut cur = 0;
    let mut ans = Vec::new();
    for i in 0..n {
        if v[i] / n != cur {
            ans.push(i);
            cur += 1;
        }
    }
    while ans.len() < n {
        ans.push(n);
    }
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
