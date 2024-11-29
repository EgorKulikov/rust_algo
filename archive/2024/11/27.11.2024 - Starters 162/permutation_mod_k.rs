//{"name":"Permutation Mod K","group":"CodeChef - START162A","url":"https://www.codechef.com/START162A/problems/PERMMODK","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 3\n1 1\n2 2\n","output":"3 4 1 2\n-1\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PermutationModK"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::iter::once;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut ans = (1..n).chain(once(0)).collect_vec();
    if (n - 1) % k == 0 {
        for i in 0..n - 1 {
            if (n - 1) % k != ans[i] % k && i % k != 0 {
                ans.swap(i, n - 1);
                out.print_line(ans.inc());
                return;
            }
        }
        out.print_line(-1);
    } else {
        out.print_line(ans.inc());
    }
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
