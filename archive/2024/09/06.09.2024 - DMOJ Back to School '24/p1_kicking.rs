//{"name":"P1 - Kicking","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p1","interactive":false,"timeLimit":1000,"tests":[{"input":"2 5 2\nA.B.A\n.BB..\n","output":"N.N.Y\n.YY..\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1Kicking"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let mut grid = input.read_char_table(n, m);

    for i in 0..n {
        let mut last = m + k;
        for j in (0..m).rev() {
            match grid[(i, j)] {
                'A' => grid[(i, j)] = if j + k < last { 'Y' } else { 'N' },
                'B' => last = j,
                _ => {}
            }
        }
        let mut last = 0;
        for j in 0..m {
            match grid[(i, j)] {
                'Y' | 'N' => last = j + k + 1,
                'B' => grid[(i, j)] = if last <= j { 'Y' } else { 'N' },
                _ => {}
            }
        }
    }
    out.print_table(&grid);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
