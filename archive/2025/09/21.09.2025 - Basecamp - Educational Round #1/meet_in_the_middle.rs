//{"name":"Meet in the middle","group":"Eolymp - Basecamp - Educational Round 1","url":"https://basecamp.eolymp.com/en/compete/1v399r4bst3f1apjnuj8as5pbc/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"4 5\n1 2 3 2\n","output":"3\n"},{"input":"6 7\n1 3 2 2 1 4\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long();
    let t = input.read_long_vec(n);

    fn subsets(arr: &[i64]) -> DefaultHashMap<i64, usize> {
        let mut ans = DefaultHashMap::new(0);
        for i in usize::iter_all(arr.len()) {
            let mut sum = 0;
            for j in arr.indices() {
                if i.is_set(j) {
                    sum += arr[j];
                }
            }
            ans[sum] += 1;
        }
        ans
    }

    let left = subsets(&t[..n / 2]);
    let right = subsets(&t[n / 2..]);

    let mut ans = 0;
    for (k, v) in left {
        ans += v * right[s - k];
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
