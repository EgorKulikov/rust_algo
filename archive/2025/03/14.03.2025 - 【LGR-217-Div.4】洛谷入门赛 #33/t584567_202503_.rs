//{"name":"T584567 [语言月赛 202503] 蛋挞制作工坊","group":"Luogu","url":"https://www.luogu.com.cn/problem/T584567?contestId=235262","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n3 5\n8 14\n4 9\n","output":"2 1\n1 2\n"},{"input":"3 2\n3 5\n8 14\n1 4\n4 9\n","output":"3 2 1\n1 3 2\n"},{"input":"2 3\n3 5 4\n6 11 8\n7 10 8\n","output":"1 2\n2 1\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let g = input.read_int_vec(m);
    let c = input.read_int_table(n, m);

    let q = Vec::with_gen(n, |i| {
        let mut res = i32::MAX;
        for j in 0..m {
            res.minim(c[(i, j)] / g[j]);
        }
        res
    });
    let rem = Arr2d::with_gen(n, m, |i, j| c[(i, j)] - q[i] * g[j]);
    for i in 0..m {
        let ans = (0..n)
            .collect::<Vec<_>>()
            .sorted_by_key(|&j| (rem[(j, i)], Reverse(q[j])));
        out.print_line(ans.inc());
    }
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
