//{"name":"Balance Difficulties","group":"CodeChef - START156A","url":"https://www.codechef.com/START156A/problems/BALDIFF","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n5 9 10\n4 1\n2 8 10 12\n1 10\n1\n","output":"1\n6 8 10\n4\n6 7 8 9\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BalanceDifficulties"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let b = input.read_long_vec(n);

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right) / 2;

        let mut from = b[0] - mid;
        let mut to = b[0] + mid;
        let mut good = true;
        for i in 1..n {
            to += x;
            from.maxim(b[i] - mid);
            to.minim(b[i] + mid);
            if from > to {
                good = false;
                break;
            }
        }
        if good {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let mut from = b.iter().map(|&x| x - left).collect::<Vec<_>>();
    for i in 1..n {
        let val = from[i - 1];
        from[i].maxim(val);
    }
    for i in (0..n - 1).rev() {
        let val = from[i + 1] - x;
        from[i].maxim(val);
    }
    out.print_line(left);
    out.print_line(from);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
