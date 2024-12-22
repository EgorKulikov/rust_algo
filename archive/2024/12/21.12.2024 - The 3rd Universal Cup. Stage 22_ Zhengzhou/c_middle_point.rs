//{"name":"C. Middle Point","group":"Universal Cup - The 3rd Universal Cup. Stage 22: Zhengzhou","url":"https://contest.ucup.ac/contest/1873/problem/9770","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n1 1\n","output":"1\n0 0 2 2\n"},{"input":"8 8\n5 0\n","output":"3\n0 0 8 0\n4 0 0 0\n2 0 8 0\n"},{"input":"0 0\n0 0\n","output":"0\n"},{"input":"2024 0\n1012 0\n","output":"1\n0 0 2024 0\n"},{"input":"2024 2024\n2023 2023\n","output":"-1\n"},{"input":"8 6\n7 3\n","output":"3\n0 0 8 0\n4 0 8 0\n6 0 8 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMiddlePoint"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int();
    let b = input.read_int();
    let mut x = input.read_int();
    let mut y = input.read_int();

    let mut ans = Vec::new();
    for _ in 0..32 {
        if (x == 0 || x == a) && (y == 0 || y == b) {
            out.print_line(ans.len());
            ans.reverse();
            out.print_per_line(&ans);
            return;
        }
        if x <= a / 2 {
            if y <= b / 2 {
                ans.push((0, 0, 2 * x, 2 * y));
                x *= 2;
                y *= 2;
            } else {
                ans.push((0, b, 2 * x, b - 2 * (b - y)));
                x *= 2;
                y = b - 2 * (b - y);
            }
        } else {
            if y <= b / 2 {
                ans.push((a, 0, a - 2 * (a - x), 2 * y));
                x = a - 2 * (a - x);
                y *= 2;
            } else {
                ans.push((a, b, a - 2 * (a - x), b - 2 * (b - y)));
                x = a - 2 * (a - x);
                y = b - 2 * (b - y);
            }
        }
    }
    out.print_line(-1);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
