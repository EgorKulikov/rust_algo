//{"name":"H. Energy Card Collection","group":"Toph","url":"https://toph.co/arena?contest=diu-take-off-fall-24-preliminary-mock#!/p/6745c26acb14b2a6cffe4972","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 2 1 3\n8 5 3 9\n1 2 3 4\n6 7 6 9\n3\n1 1 4 4\n2 2 3 3\n1 3 2 4\n","output":"73\n13\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HEnergyCardCollection"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_table(n, n);

    let mut b = Arr2d::new(n + 1, n + 1, 0);
    for i in 0..n {
        for j in 0..n {
            b[(i + 1, j + 1)] = a[(i, j)] + b[(i, j + 1)] + b[(i + 1, j)] - b[(i, j)];
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let x1 = input.read_size() - 1;
        let y1 = input.read_size() - 1;
        let x2 = input.read_size();
        let y2 = input.read_size();
        out.print_line(b[(x2, y2)] - b[(x1, y2)] - b[(x2, y1)] + b[(x1, y1)]);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
