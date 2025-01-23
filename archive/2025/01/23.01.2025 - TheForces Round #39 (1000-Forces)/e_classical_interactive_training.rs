//{"name":"E. Classical Interactive Training","group":"Codeforces - TheForces Round #39 (1000-Forces)","url":"https://codeforces.com/gym/105672/problem/E","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n3\n\n1\n\n1\n\n0\n\n4\n\n0\n\n1\n\n1\n\n1\n","output":"\n\n2 1 1 1\n\n2 1 3 2\n\n1 1 2\n\n! 2 3 1\n\n2 2 1 4\n\n2 1 2 3\n\n1 4 2\n\n! 4 3 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut query = |x: usize, y: usize, z: usize| {
        out.print_line((2, x, y, z));
        out.flush();
        input.read_int() == 1
    };
    let mut ans = vec![1, 2];
    for i in 3..=n {
        let mut l = 1;
        let mut r = ans.len();
        while l < r {
            let mid = (l + r + 1) / 2;
            if query(ans[0], ans[mid - 1], i) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        if l == 1 {
            if query(i, ans[0], ans[1]) {
                l = 0;
            }
        }
        ans.insert(l, i);
    }
    out.print_line((1, ans[0], ans[1]));
    out.flush();
    if input.read_int() == 0 {
        ans.reverse();
    }
    out.print_line(('!', ans));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
