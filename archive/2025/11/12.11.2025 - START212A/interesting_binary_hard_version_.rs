//{"name":"Interesting Binary (Hard Version)","group":"CodeChef - START212A","url":"https://www.codechef.com/START212A/problems/P5BARH","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n0 0\n2\n0 1\n4\n0 0 1 1\n3\n0 0 1\n","output":"1\n0\n6\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = Arr2d::new(3, 2, 0usize);
    let mut next = Arr2d::new(3, 2, 0usize);
    let mut ans = 0;
    let mut rem = n;
    for i in a {
        qty[(2, 0)] += 1;
        qty[(2, 1)] += qty[(2, 0)];
        qty[(2, 0)] = 0;
        next.fill(0);
        for last in 0..=2 {
            for can_save in 0..=1 {
                if i == last && (i != 1 || can_save == 0) {
                    ans += qty[(last, can_save)] * rem;
                    next[(i + 1, can_save)] += qty[(last, can_save)];
                } else {
                    next[(i, can_save)] += qty[(last, can_save)];
                }
            }
        }
        qty.fill(0);
        // if i == last && (i != 1 || !can_save) {
        //     ans += 1;
        //     last = i + 1;
        // } else {
        //     last = i;
        // }
        for last in 0..=2 {
            for can_save in 0..=1 {
                qty[(last, if i == 0 { can_save } else { 0 })] += next[(last, can_save)];
            }
        }
        rem -= 1;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    input.check_empty()
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
