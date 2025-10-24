//{"name":"Sticks","group":"Eolymp - Basecamp - Educational Round #3","url":"https://eolymp.com/en/compete/qul0nb3gg52e74o1g1rc2dm92g/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n7 1 5 2 3 2 4 5\n","output":"49\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut sums = DefaultHashMap::new(Vec::new());
    for i in usize::iter_all(n) {
        let mut sum = 0;
        for j in 0..n {
            if i.is_set(j) {
                sum += a[j];
            }
        }
        sums[sum].push(i);
    }
    let mut sides = vec![0; 1 << n];
    for (sum, mask) in sums {
        for i in mask.indices() {
            for j in 0..i {
                if (mask[i] & mask[j]) == 0 {
                    sides[mask[i] + mask[j]].maxim(sum);
                }
            }
        }
    }
    for i in usize::iter_all(n) {
        for j in 0..n {
            if i.is_set(j) {
                let cand = sides[i.without_bit(j)];
                sides[i].maxim(cand);
            }
        }
    }
    let mut ans = 0;
    for i in usize::iter_all(n) {
        let rest = usize::all_bits(n) ^ i;
        ans.maxim(sides[rest] * sides[i]);
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
