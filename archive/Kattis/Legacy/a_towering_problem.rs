//{"name":"A Towering Problem","group":"Kattis","url":"https://open.kattis.com/problems/towering","interactive":false,"timeLimit":1000,"tests":[{"input":"12 8 2 4 10 3 25 14\n","output":"12 10 3 8 4 2\n"},{"input":"12 17 36 37 51 63 92 124\n","output":"63 17 12 51 37 36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size_vec(6).sorted().reversed();
    let s1 = input.read_size();
    let _s2 = input.read_size();

    for i in usize::iter_all(6) {
        if i.count_ones() == 3 {
            let mut sum = 0;
            for j in 0..6 {
                if i.is_set(j) {
                    sum += h[j];
                }
            }
            if sum == s1 {
                let mut ans = Vec::new();
                for j in 0..6 {
                    if i.is_set(j) {
                        ans.push(h[j]);
                    }
                }
                for j in 0..6 {
                    if !i.is_set(j) {
                        ans.push(h[j]);
                    }
                }
                out.print_line(ans);
                return;
            }
        }
    }
    unreachable!();
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
