//{"name":"C. Fortitude 9.8","group":"Toph","url":"https://toph.co/arena?contest=diu-unlock-the-algorithm-fall-24-preliminary-mock#!/p/6745eb4fcb14b2a6cffe5544","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2.7 8.9 1.3 7.2 5.4\n","output":"1\n"},{"input":"9\n4.2 8.7 7.9 3.1 2.8 1.4 9.5 5.0 7.1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFortitude98"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut f = input.read_real_vec(n);

    let max = f.copy_max();
    let p = f.copy_position(|x| x == max).unwrap();
    f[p] = Real(0.0);
    let max = f.copy_max();
    let p2 = f.copy_position(|x| x == max).unwrap();
    let delta = if p > p2 { p - p2 } else { p2 - p };

    out.print_line(delta.min(n - delta) - 1);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
