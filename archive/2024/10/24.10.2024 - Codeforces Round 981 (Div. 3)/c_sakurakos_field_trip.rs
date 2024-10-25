//{"name":"C. Sakurako's Field Trip","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n5\n1 1 1 2 3\n6\n2 1 2 2 1 1\n4\n1 2 1 1\n6\n2 1 1 2 2 4\n4\n2 1 2 3\n6\n1 2 2 1 2 1\n5\n4 5 5 1 5\n7\n1 4 3 5 1 1 3\n7\n3 1 3 2 2 3 3\n","output":"1\n2\n1\n0\n0\n1\n1\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSakurakosFieldTrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut i = 1;
    let mut j = n - 2;
    let mut ans = 0;
    while i <= j {
        if a[i] == a[j] {
            if a[i] == a[i - 1] && a[i] == a[j + 1] {
                ans += 2;
            } else if a[i] == a[i - 1] || a[i] == a[j + 1] {
                ans += 1;
            }
        } else if a[i - 1] == a[j + 1] && (a[i] == a[i - 1] || a[j] == a[i - 1]) {
            ans += 1;
        }
        i += 1;
        j -= 1;
    }
    if i == j + 1 && a[i] == a[j] {
        ans += 1;
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
