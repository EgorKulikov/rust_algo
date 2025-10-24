//{"name":"S-Nim","group":"Kattis","url":"https://open.kattis.com/problems/snim","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2 5\n3\n2 5 12\n3 2 4 7\n4 2 3 7 12\n","output":"LWW\n"},{"input":"5 1 2 3 4 5\n3\n2 5 12\n3 2 4 7\n4 2 3 7 12\n","output":"WWL\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let s = input.read_size_vec(k);

    let mut ans = vec![0; 10_001];
    for i in 0..=10_000 {
        let mut options = Vec::new();
        for j in s.copy_iter() {
            if i >= j {
                options.push(ans[i - j]);
            }
        }
        options.sort();
        options.dedup();
        let mut found = false;
        for j in options.indices() {
            if options[j] != j {
                ans[i] = j;
                found = true;
                break;
            }
        }
        if !found {
            ans[i] = options.len();
        }
    }

    let m = input.read_size();
    for _ in 0..m {
        let l = input.read_size();
        let mut x = 0;
        for h in input.iter_size().take(l) {
            x ^= ans[h];
        }
        if x == 0 {
            out.print('L');
        } else {
            out.print('W');
        }
    }
    out.print_line(());
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
