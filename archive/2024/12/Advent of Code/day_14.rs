//{"name":"day_14","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_14"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::fs::File;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();
    let mut data = Vec::new();
    while !input.is_empty() {
        scan!(input, "p=@,@ v=@,@", x: i64, y: i64, dx: i64, dy: i64);
        data.push((x, y, dx, dy));
    }

    // part 1
    {
        let mut qty = Arr2d::new(2, 2, 0);
        for (x, y, dx, dy) in data.copy_iter() {
            let x = ((x + dx * 100) % n + n) % n;
            let y = ((y + dy * 100) % m + m) % m;
            if x == n / 2 || y == m / 2 {
                continue;
            }
            qty[((x / ((n + 1) / 2)) as usize, (y / ((m + 1) / 2)) as usize)] += 1;
        }
        out.print_line(qty.iter().product::<i64>());
    }

    // part 2
    {
        for i in (72..101 * 103).step_by(103) {
            let mut pic = Arr2d::new(n as usize, m as usize, b' ');
            for (x, y, dx, dy) in data.copy_iter() {
                let x = ((x + dx * i) % n + n) % n;
                let y = ((y + dy * i) % m + m) % m;
                pic[(x as usize, y as usize)] = b'#';
            }
            pic = pic.transpose();
            let mut stdout = File::create(format!("pic_{}.txt", i)).unwrap();
            let mut out = Output::new(&mut stdout);
            out.print_line(i);
            out.print_table(&pic);
            out.flush();
            // thread::sleep(std::time::Duration::from_millis(300));
        }
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
