//{"name":"C - Prepare Another Box","group":"AtCoder - AtCoder Beginner Contest 376","url":"https://atcoder.jp/contests/abc376/tasks/abc376_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2 3 7\n6 2 8\n","output":"3\n"},{"input":"4\n3 7 2 5\n8 1 6\n","output":"-1\n"},{"input":"8\n2 28 17 39 57 56 37 32\n34 27 73 28 76 61 27\n","output":"37\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrepareAnotherBox"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted();
    let b = input.read_size_vec(n - 1).sorted();

    let mut at = n;
    let mut ans = a[0];
    for i in (0..n - 1).rev() {
        while at > 0 && a[at - 1] > b[i] {
            ans = a[at - 1];
            at -= 1;
        }
        if at <= i {
            out.print_line(-1);
            return;
        }
        at -= 1;
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
