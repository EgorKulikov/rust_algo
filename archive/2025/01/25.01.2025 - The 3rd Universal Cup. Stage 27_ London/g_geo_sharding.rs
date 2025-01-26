//{"name":"G. Geo Sharding","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8617","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n","output":"1 1 1\n1 1 1\n1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Arr2d::with_gen(n, n, |i, j| {
        i / 12 + (j + 6 * (i / 12 % 2)) / 12 * n.upper_div(12) + 1
    })
    .into_iter()
    .collect::<Vec<_>>();
    let mut qty = ans.qty();
    loop {
        let mut dif = 0;
        let mut m1 = usize::MAX;
        let mut m1at = 0;
        let mut m2 = usize::MAX;
        let mut m2at = 0;
        for i in qty.indices() {
            if qty[i] != 0 {
                dif += 1;
                if qty[i] < m1 {
                    m2 = m1;
                    m2at = m1at;
                    m1 = qty[i];
                    m1at = i;
                } else if qty[i] < m2 {
                    m2 = qty[i];
                    m2at = i;
                }
            }
        }
        if dif <= 10 + n * n / 100 {
            break;
        }
        if m1 + m2 > 150 {
            panic!("Too many");
        }
        qty[m1at] = m1 + m2;
        qty[m2at] = 0;
        for i in ans.indices() {
            if ans[i] == m2at {
                ans[i] = m1at;
            }
        }
    }
    let Compressed { arrs: [ans], .. } = compress([&ans]);
    let ans = Arr2d::with_gen(n, n, |i, j| ans[i * n + j] + 1);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
