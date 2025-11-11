//{"name":"Painting the Barn (Silver)","group":"Eolymp - Basecamp - Educational Round #4","url":"https://eolymp.com/en/compete/btktopvnh51kpfku7ua54hi0bk/problem/4","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\n1 1 5 5\n4 4 7 6\n3 3 8 7\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let rect = input.read_vec::<(usize, usize, usize, usize)>(n);

    let mut d2 = Arr2d::new(1001, 1001, 0);
    for (x1, y1, x2, y2) in rect {
        d2[(x1, y1)] += 1;
        d2[(x2, y2)] += 1;
        d2[(x1, y2)] -= 1;
        d2[(x2, y1)] -= 1;
    }
    let mut delta = vec![0; 1000];
    let mut ans = 0;
    for i in 0..1000 {
        let mut cur = 0;
        for j in 0..1000 {
            delta[j] += d2[(i, j)];
            cur += delta[j];
            if cur == k {
                ans += 1;
            }
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
        TaskType::Interactive => true,
    }
}

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
