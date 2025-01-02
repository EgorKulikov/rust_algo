//{"name":"Special Array","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1152","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6\n-1 -1 1 1 1 -1\n3\n1 0 -1\n","output":"0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SpecialArray"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ones = a.copy_count(1) as i64;
    let mut zeroes = a.copy_count(0) as i64;
    let mut neg_ones = a.copy_count(-1) as i64;

    let mut ans = 0;
    while neg_ones >= 2 && ones >= 1 {
        ans += 1;
        neg_ones -= 2;
        ones -= 1;
    }
    while ones >= 3 {
        ans += 1;
        ones -= 3;
    }
    while neg_ones > 0 && zeroes > 0 {
        zeroes -= 1;
        neg_ones = (neg_ones - 2).max(0);
    }
    if neg_ones > 0 {
        ans -= neg_ones.upper_div(3);
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
