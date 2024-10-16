//{"name":"Interval Cliques","group":"CodeChef - START156A","url":"https://www.codechef.com/START156A/problems/INTCLIQUE","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 4\n2 3\n5 6\n5\n1 9\n2 4\n3 7\n5 6\n8 10\n1\n1 2\n","output":"2\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IntervalCliques"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let segements = input.read_int_pair_vec(n).sorted();

    let mut a = Vec::with_capacity(n);
    for (_, r) in segements {
        let pos = a.lower_bound(&r);
        if pos == a.len() {
            a.push(r);
        } else {
            a[pos] = r;
        }
    }
    out.print_line(a.len());
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
