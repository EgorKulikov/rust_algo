//{"name":"E. Kevin and Bipartite Graph","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2\n3 7\n5 4\n","output":"YES\n1 2\n2 1\n2 2\n2 1\nNO\nYES\n1 1 1 1\n1 2 2 2\n1 2 3 3\n1 2 3 4\n1 2 3 4\n1 2 3 4\n1 2 3 4\n1 2 3 4\n1 2 3 4\n1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKevinAndBipartiteGraph"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    if m >= 2 * n {
        out.print_line(false);
        return;
    }
    let mut ans = Arr2d::new(2 * n, m, 0);
    for i in 0..n {
        for j in 2 * i..2 * i + m {
            ans[(j % (2 * n), j - 2 * i)] = i + 1;
            ans[((j + 1) % (2 * n), j - 2 * i)] = i + 1;
        }
    }
    out.print_line(true);
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
