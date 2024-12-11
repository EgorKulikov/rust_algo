//{"name":"L. Last Chance: Threads of Despair","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9873","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n1 1 4\n2 6\n3 2\n1 1 4\n2 7\n2 1\n100 100\n2\n","output":"Yes\nNo\nYes\n"},{"input":"3\n7 1\n1 1 1 1 1 1 1\n9\n5 2\n3 4 5 6 7\n1 6\n5 3\n3 4 5 6 7\n1 5 7\n","output":"No\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LLastChanceThreadsOfDespair"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut h0 = input.read_size_vec(n).sorted();
    let mut h1 = input.read_size_vec(m).sorted();

    let mut level = 0;
    let mut shots = if h0[0] == 1 { 1 } else { 0 };
    let mut j = 0;
    let mut k = 0;
    for i in 0..n {
        if h0[i] != 1 {
            shots += 1;
        }
        h0[i] -= 1;
    }
    loop {
        if j < m && h1[j] <= level {
            j += 1;
            level += 1;
        } else if k < n && h0[k] <= level {
            k += 1;
            level += 1;
        } else {
            break;
        }
    }
    while j < m {
        if shots == 0 {
            out.print_line(false);
            return;
        }
        h1[j] -= 1;
        shots -= 1;
        loop {
            if j < m && h1[j] <= level {
                j += 1;
                level += 1;
            } else if k < n && h0[k] <= level {
                k += 1;
                level += 1;
            } else {
                break;
            }
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
