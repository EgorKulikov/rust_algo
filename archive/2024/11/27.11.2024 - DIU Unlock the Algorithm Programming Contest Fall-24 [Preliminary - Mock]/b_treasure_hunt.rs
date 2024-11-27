//{"name":"B. Treasure Hunt","group":"Toph","url":"https://toph.co/arena?contest=diu-unlock-the-algorithm-fall-24-preliminary-mock#!/p/6745eb35cb14b2a6cffe5534","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3 3 3\n2 3 4 5\n3 2 1 3\n4 5 2 5\n","output":"32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTreasureHunt"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let tr = input.read_size_table(n, n);

    let mut ans = Arr2d::new(n, n, 0);
    for i in 0..n {
        for j in 0..n {
            ans[(i, j)] = tr[(i, j)];
            if i > 0 {
                let cand = ans[(i - 1, j)] + tr[(i, j)];
                ans[(i, j)].maxim(cand);
            }
            if j > 0 {
                let cand = ans[(i, j - 1)] + tr[(i, j)];
                ans[(i, j)].maxim(cand);
            }
        }
    }
    out.print_line(ans[(n - 1, n - 1)] + 2 * n - 2);
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
