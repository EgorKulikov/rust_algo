//{"name":"H. Maximize Array","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9483","interactive":false,"timeLimit":1000,"tests":[{"input":"9 3\n1 2 3 4 1 2 3 4 1\n","output":"4 4 1\n"},{"input":"6 1\n1 6 4 2 3 5\n","output":"6 5\n"},{"input":"6 5\n6 5 4 3 2 1\n","output":"6 5 4 3 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMaximizeArray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut max = vec![0; n];
    for i in (0..n).rev() {
        max[i] = a[i];
        if i + k < n {
            let next = max[i + k];
            max[i].maxim(next);
        }
    }
    let mut ans = Vec::new();
    let mut pos = 0;
    while pos < n {
        if a[pos] == max[pos] {
            ans.push(a[pos]);
            pos += 1;
        } else {
            pos += k;
        }
    }
    out.print_line(ans);
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
