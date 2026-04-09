//{"name":"AB or C","group":"CodeChef - START233A","url":"https://www.codechef.com/START233A/problems/ABC7","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n4\n2 4 8\n10 12 7\n1 6 3\n8 2 3\n4\n10 7 7\n2 5 5\n3 9 9\n11 12 12\n","output":"2\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut abc = input.read_vec::<[i32; 3]>(n);

    for i in 0..n {
        abc[i].sort();
    }
    let mut pos = vec![0; n];
    let mut order = Vec::with_capacity(3 * n);
    let mut max = 0;
    for i in 0..n {
        for j in 0..3 {
            order.push((abc[i][j], i));
        }
        max.maxim(abc[i][0]);
    }
    order.sort();
    let mut ans = None;
    for (val, i) in order {
        ans.minim(max - val);
        if pos[i] == 2 {
            break;
        }
        pos[i] += 1;
        max.maxim(abc[i][pos[i]]);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
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
