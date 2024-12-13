//{"name":"day_13","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_13"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        scan!(input, "Button A: X+@, Y+@", x0: i64, y0: i64);
        scan!(input, "Button B: X+@, Y+@", x1: i64, y1: i64);
        scan!(input, "Prize: X=@, Y=@", x: i64, y: i64);
        data.push((x0, y0, x1, y1, x, y));
    }

    // part 1
    {
        let mut ans = 0;
        for (x0, y0, x1, y1, x, y) in data.copy_iter() {
            let mut cur = None;
            for a in 0..=100 {
                for b in 0..=100 {
                    let xx = x0 * a + x1 * b;
                    let yy = y0 * a + y1 * b;
                    if xx == x && yy == y {
                        cur.minim(3 * a + b);
                    }
                }
            }
            if let Some(cur) = cur {
                ans += cur;
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for (x0, y0, x1, y1, mut x, mut y) in data {
            x += 10000000000000;
            y += 10000000000000;
            let d = x0 * y1 - x1 * y0;
            let dy = x * y0 - y * x0;
            if dy % d != 0 {
                continue;
            }
            let b = -dy / d;
            let dx = x * y1 - x1 * y;
            if dx % d != 0 {
                continue;
            }
            let a = dx / d;
            if a < 0 || b < 0 {
                continue;
            }
            ans += 3 * a + b;
        }
        out.print_line(ans);
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
