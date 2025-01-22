//{"name":"Array Compression","group":"CodeChef - START170A","url":"https://www.codechef.com/START170A/problems/RACMP","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6\n1 5 2 2 1 6\n9\n5 5 5 7 7 7 3 3 3\n4\n1 3 1 2\n","output":"4\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n).sorted();
    let q = qty(&a);
    let mut singles = 0;
    let mut non2 = 0;
    let mut non3 = 0;
    for v in q.into_values() {
        if v == 1 {
            singles += 1;
        } else {
            if v % 2 != 0 {
                non2 += 1;
            }
            if v % 3 != 0 {
                non3 += 1;
            }
        }
    }
    if singles == n {
        out.print_line(n);
    } else if non2 == 0 {
        out.print_line(singles + 2);
    } else if non3 == 0 || non2 == 1 {
        out.print_line(singles + 3);
    } else if non2 == 2 || non3 == 1 {
        out.print_line(singles + 4);
    } else {
        out.print_line(singles + 5);
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
