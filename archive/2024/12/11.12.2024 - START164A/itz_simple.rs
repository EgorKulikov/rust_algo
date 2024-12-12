//{"name":"Itz Simple","group":"CodeChef - START164A","url":"https://www.codechef.com/START164A/problems/SPC2025Q2","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 4 2\n1 3\n6 10 6\n6 1 4 5 7 8\n4 10 6\n1 2 7 5\n3 2 4\n1 1 4\n","output":"Ved\nVarun\nVed\nEqual\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ItzSimple"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let k = k + a[Back(0)];
    let p = p + a.iter_take(n - 1).sum::<usize>();
    out.print_line(match k.cmp(&p) {
        std::cmp::Ordering::Greater => "Ved",
        std::cmp::Ordering::Less => "Varun",
        std::cmp::Ordering::Equal => "Equal",
    })
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
