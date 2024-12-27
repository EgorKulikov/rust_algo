//{"name":"Yoda","group":"Kattis","url":"https://open.kattis.com/problems/yoda","interactive":false,"timeLimit":1000,"tests":[{"input":"300\n500\n","output":"0\n500\n"},{"input":"65743\n9651\n","output":"673\n95\n"},{"input":"2341\n6785\n","output":"YODA\n6785\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Yoda"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Ordering;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_size();
    let mut m = input.read_size();

    let mut a1 = 0;
    let mut t1 = 1;
    let mut a2 = 0;
    let mut t2 = 1;
    while n > 0 || m > 0 {
        let d1 = n % 10;
        let d2 = m % 10;
        n /= 10;
        m /= 10;
        match d1.cmp(&d2) {
            Ordering::Less => {
                a2 += d2 * t2;
                t2 *= 10;
            }
            Ordering::Equal => {
                a1 += d1 * t1;
                a2 += d2 * t2;
                t1 *= 10;
                t2 *= 10;
            }
            Ordering::Greater => {
                a1 += d1 * t1;
                t1 *= 10;
            }
        }
    }
    if t1 == 1 {
        out.print_line("YODA");
    } else {
        out.print_line(a1);
    }
    if t2 == 1 {
        out.print_line("YODA");
    } else {
        out.print_line(a2);
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
