//{"name":"Annihilator Gun","group":"Eolymp - Basecamp - Eolymp Cup #2","url":"https://basecamp.eolymp.com/en/compete/ptmnufrm6p6nl7gods1loo65go/problem/1","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n1 2\n","output":"1\n"},{"input":"2 2\n1 2\n","output":"-1\n"},{"input":"3 9\n9 9 9\n","output":"0\n"},{"input":"4 3\n1 2 3 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AnnihilatorGun"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();

    if a.iter().count_eq(&&a[0]) == n {
        out.print_line(0);
        return;
    }
    if k <= 2 {
        out.print_line(-1);
        return;
    }
    let mut free = BitSet::new(k);
    free.fill(true);
    for i in a {
        free.unset(i);
    }
    if free.iter().next().is_some() {
        out.print_line(1);
    } else {
        out.print_line(2);
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
