//{"name":"day_11","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_11"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::{NumDigs, Power};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let stones = input.iter::<i64>().collect_vec();

    let mut mem = Memoization2::new(|mem, n: i64, rem: usize| -> i64 {
        if rem == 0 {
            1
        } else if n == 0 {
            mem.call(1, rem - 1)
        } else if n.num_digs() % 2 == 0 {
            let ten = 10.power(n.num_digs() / 2);
            mem.call(n / ten, rem - 1) + mem.call(n % ten, rem - 1)
        } else {
            mem.call(n * 2024, rem - 1)
        }
    });
    // port 1
    {
        let mut ans = 0;
        for i in stones.copy_iter() {
            ans += mem.call(i, 25);
        }
        out.print_line(ans);
    }

    // port 2
    {
        let mut ans = 0;
        for i in stones.copy_iter() {
            ans += mem.call(i, 75);
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
