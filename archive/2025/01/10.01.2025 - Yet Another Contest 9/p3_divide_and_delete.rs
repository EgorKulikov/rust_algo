//{"name":"P3 - Divide and Delete","group":"DMOJ - Yet Another Contest 9","url":"https://dmoj.ca/problem/yac9p3","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 6 1\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3DivideAndDelete"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let max = a.copy_max();
    let dt = divisor_table(max + 1);

    let mut left = vec![n; n];
    let mut right = vec![n; n];
    let mut add = vec![Vec::new(); n];
    let mut ans = 0;
    for i in (0..n).rev() {
        let mut cur = 1;
        let mut a = a[i];
        while a > 1 {
            cur.maxim(dt[a]);
            a /= dt[a];
        }
        if cur <= i + 1 {
            add[i + 1 - cur].push(i);
        }
        for j in add[i].drain(..) {
            left[j] = j;
            right[j] = j;
            if j > 0 && left[j - 1] != n {
                left[j] = left[j - 1];
                right[left[j]] = j;
            }
            if j + 1 < n && right[j + 1] != n {
                let end = right[j + 1];
                let start = left[j];
                left[end] = start;
                right[start] = end;
            }
        }
        if right[i] != n {
            ans += right[i] - i + 1;
        }
    }
    out.print_line(ans);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
