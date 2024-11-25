//{"name":"Harvesting Rice","group":"TLX - TLX Regular Open Contest #41","url":"https://tlx.toki.id/contests/troc-41/problems/D","interactive":false,"timeLimit":1000,"tests":[{"input":"9 6\n10 7 4 5 9 7 4 1 7\n","output":"13\n"},{"input":"2 1\n1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HarvestingRice"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_long_vec(n);

    a[..=k].sort();
    let mut b = a.clone();
    a[1] += a[0];
    a[0] = 0;
    b[k] += b[0];
    b[0] = 0;
    a.sort();
    b.sort();
    let ans1 = a.copy_take(n / 2).sum::<i64>();
    let ans2 = b.copy_take(n / 2).sum::<i64>();
    out.print_line(ans1.max(ans2));
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
