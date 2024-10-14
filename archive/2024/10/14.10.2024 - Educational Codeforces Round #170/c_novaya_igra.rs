//{"name":"C. Новая игра","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n10 2\n5 2 4 3 4 3 4 5 3 2\n5 1\n10 11 10 11 10\n9 3\n4 5 4 4 6 5 4 4 6\n3 2\n1 3 1\n","output":"6\n3\n9\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNovayaIgra"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n).sorted();

    let mut j = 0;
    let mut ans = 0;
    for i in 0..n {
        if i > 0 && a[i] - a[i - 1] > 1 {
            j = i;
        }
        while a[i] - a[j] >= k {
            j += 1;
        }
        ans.maxim(i - j + 1);
    }
    out.print_line(ans);
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
