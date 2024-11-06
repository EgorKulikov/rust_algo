//{"name":"Elections","group":"CodeChef - START159A","url":"https://www.codechef.com/START159A/problems/USELEC","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 1\n2\n3\n3 2\n2 2 3\n1 3 2\n3 3\n3 1 5\n3 2 7\n","output":"NO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Elections"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let need = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| (b + 1).saturating_sub(a))
        .collect_vec()
        .sorted();
    out.print_line(x >= need.into_iter().take((n + 1) / 2).sum::<usize>());
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
